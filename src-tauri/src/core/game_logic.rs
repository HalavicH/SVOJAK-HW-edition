use std::thread::sleep;
use std::time::Duration;
use error_stack::{IntoReport, ResultExt, Result, Report};
use rand::Rng;
use crate::core::game_entities::{GameContext, GamePackError, GameplayError, Player, PlayerState};
use crate::core::hub_manager::{get_epoch_ms, HubManagerError};
use crate::game_pack::pack_content_entities::{Question, Round};

impl GameContext {
    pub fn get_question(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), GamePackError> {
        log::info!("Get question from category: {theme}, price: {price}");
        let round = self.get_current_round_mut();

        let theme = round.themes.get_mut(theme)
            .ok_or(GamePackError::ThemeNotPresent).into_report()
            .attach_printable(format!("Can't find theme: {theme:?}"))?;

        let question = theme.get_question(price)
            .ok_or(GamePackError::QuestionNotPresent).into_report()
            .attach_printable(format!("Can't find question with price {price:?} in theme: {theme:?}"))?
            .clone();

        let question_number = round.question_count - round.questions_left;
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
        self.current.answer_allowed = true;
        self.hub.allow_answer_timestamp = get_epoch_ms()?;
        Ok(())
    }

    pub fn get_fastest_click(&mut self) -> Result<u8, GameplayError> {
        // TODO: Add logic for fastest click

        let keys = self.players.keys()
            .map(|k| k.clone())
            .collect::<Vec<u8>>();

        sleep(Duration::from_millis(200));

        let fastest_click_index: usize = rand::thread_rng().gen_range(0..keys.len() as usize);
        let fastest_player_id = self.players.get(&keys[fastest_click_index])
            .ok_or(Report::new(GameplayError::PlayerNotPresent))?
            .term_id;

        log::info!("Fastest click from user: {}", fastest_player_id);
        self.current.answer_allowed = false;
        self.current.active_player_id = fastest_player_id;

        self.players.get_mut(&fastest_player_id)
            .ok_or(Report::new(GameplayError::PlayerNotPresent))?
            .state = PlayerState::FirstResponse;

        Ok(fastest_player_id)
    }

    pub fn get_active_player_id(&self) -> u8 {
        self.current.active_player_id
    }

    pub fn answer_question(&mut self, answered_correctly: bool) -> Result<Player, GameplayError> {
        let player_id = self.get_active_player_id();

        let player = {
            let player = self.players.get_mut(&player_id)
                .ok_or(GameplayError::PlayerNotPresent)?;
            if answered_correctly {
                player.stats.correct_num += 1;
            } else {
                player.stats.wrong_num += 1;
            }
            player.stats.total_tries += 1;
            player.clone()
        };

        let theme = self.current.question_theme.clone();
        let price = self.current.question_price.clone();

        if self.no_players_to_answer_left() {
            self.remove_question(&theme, &price)
                .change_context(GameplayError::PackElementNotPresent)?;
        }

        Ok(player.clone())
    }

    pub fn get_current_round(&self) -> &Round {
        let index = self.current.round_index as usize;
        let round = self.pack.rounds.get(index).unwrap();
        &round
    }

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current.round_index as usize;
        let round = self.pack.rounds.get_mut(index).unwrap();
        round
    }

    fn no_players_to_answer_left(&mut self) -> bool {
        let players_left = self.players.iter()
            .filter(|(_, &ref player)| player.state != PlayerState::Inactive && player.state != PlayerState::Dead)
            .count();
        players_left == 0
    }
}