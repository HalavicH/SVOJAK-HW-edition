use std::thread::sleep;
use std::time::Duration;
use error_stack::{IntoReport, ResultExt, Result, Report};
use log::log;
use rand::Rng;
use crate::api::dto::QuestionType;
use crate::core::game_entities::{GameContext, GamePackError, GameplayError, GameState, Player, PlayerState};
use crate::core::hub_manager::{get_epoch_ms, HubManagerError};
use crate::game_pack::pack_content_entities::{Question, Round, RoundType};

impl GameContext {
    pub fn get_question(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), GamePackError> {
        log::info!("Get question from category: {theme}, price: {price}");
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
        self.current.question_price = question.price.clone();

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
        self.get_current_round().questions_left > 0
    }

    pub fn allow_answer(&mut self) -> Result<(), HubManagerError> {
        let timestamp = get_epoch_ms()?;
        self.hub.allow_answer_timestamp = timestamp;
        log::info!("Current answer base timestamp: {timestamp}");

        self.update_non_target_player_states();
        self.current.click_for_answer_allowed = true;
        Ok(())
    }

    pub fn get_fastest_click(&mut self) -> Result<u8, GameplayError> {
        // TODO: Add logic for fastest click

        let keys = self.players.iter()
            .filter(|(_, &ref p)| { p.allowed_to_click() })
            .map(|(id, _)| id.clone())
            .collect::<Vec<u8>>();

        if keys.is_empty() {
            let report = Report::new(GameplayError::OperationForbidden)
                .attach_printable("Can't get first click: No players allowed to click left.");
            return Err(report);
        }

        log::info!("Users participating in the click race: {:?}", keys);

        sleep(Duration::from_millis(200));

        let fastest_click_index: usize = rand::thread_rng().gen_range(0..keys.len() as usize);
        let fastest_player_id = keys[fastest_click_index];

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
        self.current.get_active_player_id()
    }

    pub fn answer_question(&mut self, answered_correctly: bool) -> Result<Player, GameplayError> {
        if self.current.answer_allowed == false {
            return Err(Report::new(GameplayError::AnswerForbidden))
        }

        self.current.answer_allowed = false;

        let active_player_id = self.get_active_player_id();
        log::info!("Active player id: {}. Player ids: {:?}", active_player_id,self.get_player_keys());

        let response_player = {
            let active_player = self.players.get_mut(&active_player_id)
                .ok_or(GameplayError::PlayerNotPresent)?;
            if answered_correctly {
                active_player.stats.correct_num += 1;
                active_player.stats.score += self.current.question_price;
                active_player.state = PlayerState::AnsweredCorrectly;
            } else {
                active_player.stats.wrong_num += 1;
                active_player.stats.score -= self.current.question_price;
                active_player.state = PlayerState::AnsweredWrong;
            }
            active_player.stats.total_tries += 1;
            active_player.clone()
        };

        log::info!("Current player stats: {:?}", response_player);

        let theme = self.current.question_theme.clone();
        let price = self.current.question_price.clone();

        if answered_correctly || self.no_players_to_answer_left() {
            log::info!("Removing question from the pack");
            self.remove_question(&theme, &price)
                .change_context(GameplayError::PackElementNotPresent)?;

            if !self.has_next_question() {
                log::info!("No question left for round {}. Initializing new round",
                    self.get_current_round().name);
                self.init_next_round()
            }
        }

        Ok(response_player)
    }

    pub fn get_current_round(&self) -> &Round {
        let index = self.current.round_index;
        let round = self.pack.rounds.get(index).unwrap();
        &round
    }

    fn get_player_keys(&self) -> Vec<u8> {
        self.players.keys()
            .map(|k| k.clone())
            .collect()
    }

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current.round_index;
        let round = self.pack.rounds.get_mut(index).unwrap();
        round
    }

    fn no_players_to_answer_left(&mut self) -> bool {
        let players_left = self.players.iter()
            .filter(|(_, &ref player)| {
                player.state != PlayerState::Inactive && player.state != PlayerState::Dead
            })
            .count();
        players_left == 0
    }

    fn update_non_target_player_states(&mut self) {
        let game_state = self.current.game_state();
        self.players.iter_mut().for_each(|(id, p)| {
            if p.state == PlayerState::AnsweredWrong {
                log::info!("Player with id {} becomes inactive", id);
                p.state = PlayerState::Inactive;
            }
            if *game_state == GameState::QuestionSelection && p.state == PlayerState::AnsweredCorrectly {
                log::info!("Player with id {} becomes idle", id);
                p.state = PlayerState::Idle;
            }
        });
    }

    fn init_next_round(&mut self) {
        if (self.pack.rounds.len() - 1) == self.current.round_index as usize {
            log::error!("Already final round");
            return;
        }
        self.current.round_index += 1;
        let round: &Round = self.pack.rounds.get(self.current.round_index).expect("Round should be present");
        log::info!("Next round name {}", round.name);

        if round.round_type == RoundType::Final {
            self.kill_players_with_negative_balance();
        }
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