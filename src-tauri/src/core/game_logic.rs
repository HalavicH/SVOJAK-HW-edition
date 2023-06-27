use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use error_stack::{IntoReport, ResultExt, Result, Report};
use rand::Rng;
use crate::api::dto::{PlayerStatsDto, RoundStatsDto};
use crate::core::game_entities::{GameContext, GamePackError, GameplayError, GameState, Player, PlayerState};
use crate::core::hub_manager::{get_epoch_ms, HubManager, HubManagerError};
use crate::game_pack::pack_content_entities::{Question, Round, RoundType};
use crate::hw_comm::api_types::TermEvent;

const EVT_POLLING_INTERVAL_MS: u64 = 100;

impl GameContext {
    pub fn start_the_game(&mut self) -> Result<(), GameplayError> {
        self.update_game_state(GameState::QuestionChoosing);

        if self.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::PlayerNotPresent).into_report();
        }

        start_event_listener(self.get_hub_ref().clone());

        let q_picker_id = match self.get_fastest_click_player_id() {
            Ok(id) => { id }
            Err(err) => {
                log::error!("{:#?}", err);

                self.players.values().next()
                    .ok_or(GameplayError::PlayerNotPresent).into_report()
                    .attach_printable("Can't find any player to play with :D")?
                    .term_id
            }
        };

        self.current.set_active_player_id(q_picker_id);
        let player = self.players.get_mut(&self.current.active_player_id())
            .ok_or(GameplayError::PlayerNotPresent).into_report()?;
        player.state = PlayerState::QuestionChooser;
        Ok(())
    }

    pub fn fetch_players(&mut self) -> &HashMap<u8, Player> {
        self.update_non_target_player_states();
        &self.players
    }

    pub fn process_question_obtaining(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), GamePackError> {
        log::info!("Get question from category: {theme}, price: {price}");
        let (question, question_number) = self.get_question(theme, price)?;

        self.update_game_state(GameState::QuestionSelected);

        Ok((question, question_number))
    }

    pub fn remove_question(&mut self, theme: &String, price: &i32) -> Result<(), GamePackError> {
        log::info!("Try to remove question from category: {theme}, price: {price}");
        let round = self.get_current_round_mut();
        let theme = round.themes.get_mut(theme)
            .ok_or(GamePackError::ThemeNotPresent).into_report()
            .attach_printable(format!("Can't find theme: {theme:?}"))?;

        let _ = theme.pop_question(price)
            .ok_or(GamePackError::QuestionNotPresent).into_report()
            .attach_printable(format!("Can't find question with price {price:?} in theme: {theme:?}"))?;

        round.questions_left -= 1;
        log::info!("Question left: {}", round.questions_left);
        Ok(())
    }

    pub fn has_next_question(&self) -> bool {
        // self.current.has_next_question
        let has_new_question = self.get_current_round().questions_left > 0;
        log::info!("Has new question: {}", has_new_question);
        has_new_question
    }

    pub fn allow_answer(&mut self) -> Result<(), HubManagerError> {
        let timestamp = get_epoch_ms()?;
        self.get_unlocked_hub_mut().allow_answer_timestamp = timestamp;
        log::info!("Current answer base timestamp: {timestamp}");

        self.current.set_active_player_id(0);
        self.update_non_target_player_states();
        self.current.click_for_answer_allowed = true;
        Ok(())
    }

    pub fn get_fastest_click_player_id(&mut self) -> Result<u8, GameplayError> {
        let players_allowed_to_click_num = self.players.values()
            .filter(|&p| { p.allowed_to_click() })
            .count();
        if players_allowed_to_click_num == 0 {
            let report = Report::new(GameplayError::OperationForbidden)
                .attach_printable("Can't get first click: No players allowed to click left.");
            return Err(report);
        }

        let fastest_player_id = self.get_fastest_click_from_hub()
            .change_context(GameplayError::HubOperationError)?;

        log::info!("Fastest click from user: {}", fastest_player_id);
        self.current.click_for_answer_allowed = false;
        self.current.answer_allowed = true;
        self.current.set_active_player_id(fastest_player_id);

        self.players.get_mut(&fastest_player_id)
            .ok_or(Report::new(GameplayError::PlayerNotPresent))
            .attach_printable(format!("Can't find player with id {}", fastest_player_id))?
            .state = PlayerState::FirstResponse;

        Ok(fastest_player_id)
    }


    pub fn get_active_player_id(&self) -> u8 {
        self.current.active_player_id()
    }

    pub fn answer_question(&mut self, answered_correctly: bool) -> Result<bool, GameplayError> {
        if !self.current.answer_allowed {
            return Err(Report::new(GameplayError::AnswerForbidden));
        }

        self.current.answer_allowed = false;

        let active_player_id = self.get_active_player_id();
        log::info!("Active player id: {}. Player ids: {:?}", active_player_id, self.get_player_keys());

        let response_player = {
            let active_player = self.players.get_mut(&active_player_id)
                .ok_or(GameplayError::PlayerNotPresent)?;
            if answered_correctly {
                active_player.stats.correct_num += 1;
                self.current.total_correct_answers += 1;
                active_player.stats.score += self.current.question_price;
                active_player.state = PlayerState::AnsweredCorrectly;
            } else {
                active_player.stats.wrong_num += 1;
                active_player.stats.score -= self.current.question_price;
                active_player.state = PlayerState::AnsweredWrong;
            }
            self.current.total_tries += 1;
            active_player.stats.total_tries += 1;
            active_player.clone()
        };

        log::info!("Current player stats: {:?}", response_player);

        if self.no_players_to_answer_left() {
            log::info!("Nobody answered question correctly");
            self.current.total_wrong_answers += 1;
        }

        let theme = self.current.question_theme.clone();
        let price = self.current.question_price;

        let mut retry = true;
        if answered_correctly || self.no_players_to_answer_left() {
            log::info!("Removing question from the pack");

            retry = false;
            self.current.set_active_player_id(0);
            self.update_game_state(GameState::QuestionChoosing);
            self.update_non_target_player_states();

            self.remove_question(&theme, &price)
                .change_context(GameplayError::PackElementNotPresent)?;
        }

        Ok(retry)
    }

    pub fn get_current_round(&self) -> &Round {
        let index = self.current.round_index;
        let round = self.game_pack.content.rounds.get(index).unwrap();
        round
    }

    pub fn no_players_to_answer_left(&self) -> bool {
        let players_left = self.players.iter()
            .filter(|(_, p)| {
                p.state != PlayerState::Inactive
                    && p.state != PlayerState::Dead
                    && p.state != PlayerState::AnsweredWrong
            })
            .count();
        log::debug!("Players to answer left: {}", players_left);
        players_left == 0
    }

    pub fn init_next_round(&mut self) {
        if (self.game_pack.content.rounds.len() - 1) == self.current.round_index {
            log::error!("Already final round");
            return;
        }
        self.current.round_index += 1;
        let round: &Round = self.game_pack.content.rounds.get(self.current.round_index).expect("Round should be present");
        log::info!("Next round name {}", round.name);

        self.current.total_tries = 0;
        self.current.total_wrong_answers = 0;
        self.current.total_correct_answers = 0;

        if round.round_type == RoundType::Final {
            self.kill_players_with_negative_balance();
        }
    }

    pub fn fetch_round_stats(&self) -> RoundStatsDto {
        let round = self.get_current_round();
        RoundStatsDto {
            roundName: round.name.to_owned(),
            questionNumber: round.question_count,
            normalQuestionNum: round.normal_question_count,
            pigInPokeQuestionNum: round.pip_question_count,
            totalCorrectAnswers: self.current.total_correct_answers,
            totalWrongAnswers: self.current.total_wrong_answers,
            totalTries: self.current.total_tries,
            roundTime: "Not tracked".to_owned(),
            players: self.players.values()
                .map(|p| {
                    PlayerStatsDto {
                        id: p.term_id as i32,
                        name: p.name.to_owned(),
                        score: p.stats.score,
                        playerIconPath: p.icon.to_owned(),
                        totalAnswers: p.stats.total_tries,
                        answeredCorrectly: p.stats.correct_num,
                        answeredWrong: p.stats.wrong_num,
                    }
                })
                .collect(),
        }
    }

    fn update_game_state(&mut self, new_state: GameState) {
        log::info!("Game state {:?} -> {:?}", self.current.game_state(), new_state);
        self.current.set_game_state(new_state);
        self.update_non_target_player_states();
    }

    fn get_question(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), GamePackError> {
        let round = self.get_current_round_mut();
        let question_number = round.question_count - round.questions_left;

        let theme = round.themes.get_mut(theme)
            .ok_or(GamePackError::ThemeNotPresent).into_report()
            .attach_printable(format!("Can't find theme: {theme:?}"))?;

        let question = theme.get_question(price)
            .ok_or(GamePackError::QuestionNotPresent).into_report()
            .attach_printable(format!("Can't find question with price {price:?} in theme: {theme:?}"))?
            .clone();

        self.current.question_theme = theme.name.clone();
        self.current.question_type = question.question_type.clone();
        self.current.question_price = question.price;
        Ok((question, question_number))
    }

    fn get_fastest_click_from_hub(&mut self) -> Result<u8, HubManagerError> {
        // let mut events = vec![];
        // while events.is_empty() {
        //     events = self.get_hub().read_event_queue()?;
        // }
        //
        // events.iter()
        //     .filter(|e| e.state == TermButtonState::Pressed)
        // for evt in events {
        //     evt.
        // }

        // TODO: Add logic for fastest click

        let keys = self.players.iter()
            .filter(|(_, p)| { p.allowed_to_click() })
            .map(|(id, _)| *id)
            .collect::<Vec<u8>>();

        log::debug!("Users participating in the click race: {:?}", keys);

        sleep(Duration::from_millis(200));

        let fastest_click_index: usize = rand::thread_rng().gen_range(0..keys.len());
        let fastest_player_id = keys[fastest_click_index];
        Ok(fastest_player_id)
    }

    fn get_player_keys(&self) -> Vec<u8> {
        self.players.keys().copied()
            .collect()
    }

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current.round_index;
        let round = self.game_pack.content.rounds.get_mut(index).unwrap();
        round
    }

    fn update_non_target_player_states(&mut self) {
        let game_state = self.current.game_state();
        let active_id = self.get_active_player_id();

        self.players.iter_mut().for_each(|(id, p)| {
            log::debug!("Game state: {:?}. Player: {}:{:?}", game_state, p.term_id, p.state);

            if p.term_id == active_id {
                log::debug!("Active player. Skipping");
                return;
            }

            if p.state == PlayerState::AnsweredWrong {
                log::trace!("Player with id {} becomes inactive", id);
                p.state = PlayerState::Inactive;
            }

            if *game_state == GameState::QuestionChoosing || (p.state != PlayerState::Dead && p.state != PlayerState::Inactive) {
                log::trace!("Player with id {} becomes idle", id);
                p.state = PlayerState::Idle;
            }
        });
    }

    fn kill_players_with_negative_balance(&mut self) {
        self.players.iter_mut().for_each(|(_, player)| {
            if player.stats.score < 0 {
                log::info!("Killing player {:?} because of the negative balance", player);
                player.state = PlayerState::Dead;
            }
        });
    }
}

pub fn start_event_listener(hub: Arc<RwLock<HubManager>>) -> JoinHandle<()> {
    log::info!("Starting event listener");

    thread::spawn(move || {
        listen_hub_events(hub);
    })
}

fn listen_hub_events(hub: Arc<RwLock<HubManager>>) {
    let hub_guard = hub.read().expect("Mutex is poisoned");
    loop {
        log::debug!("############# NEW ITERATION ###############");
        sleep(Duration::from_millis(EVT_POLLING_INTERVAL_MS));
        let events = hub_guard.read_event_queue()
            .unwrap_or_else(|error| {
                log::error!("Can't get events. Err {:?}", error);
                vec![]
            });

        if events.is_empty() {
            log::debug!("No player events occurred");
            continue;
        }

        events.iter()
            .for_each(|e| {
                process_term_event(&hub_guard, e);
            });
    }
}

fn process_term_event(hub_guard: &RwLockReadGuard<HubManager>, e: &TermEvent) {
    hub_guard.set_term_feedback_led(e.term_id, &e.state)
        .unwrap_or_else(|error| {
            log::error!("Can't set term_feedback let. Err {:?}", error);
        });

    if e.timestamp >= hub_guard.allow_answer_timestamp {
        log::info!("After answer allowed");
    } else {
        log::info!("Forbidden. Adding 1s delay for the answer");
    }
}

