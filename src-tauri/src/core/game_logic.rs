use rand::Rng;
use crate::core::game_entities::GameContext;
use crate::game_pack::pack_content_entities::{Question, Round};

impl GameContext {
    pub fn get_current_round(&self) -> &Round {
        let index = self.current_round_index as usize;
        let round = self.pack.rounds.get(index).unwrap();
        &round
    }

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current_round_index as usize;
        let round = self.pack.rounds.get_mut(index).unwrap();
        round
    }

    pub fn pop_question(&mut self, theme: &String, price: &i32) -> Result<(Question, i32), String> {
        log::info!("Get question from category: {theme}, price: {price}");
        let round = self.get_current_round_mut();
        let theme = round.themes.get_mut(theme).ok_or("Theme not found".to_string())?;
        let question = theme.pop_question(price).ok_or("Question not found".to_string())?.clone();
        round.questions_left -= 1;
        log::info!("Question left: {}", round.questions_left);

        let question_number = round.question_count - round.questions_left;
        Ok((question, question_number))
    }

    pub fn has_next_question(&self) -> bool {
        self.get_current_round().questions_left > 0
    }

    pub fn get_fastest_click(&self) -> i32 {
        // TODO: Add logic for fastest click

        let random_int = rand::thread_rng().gen_range(1..100);
        log::info!("Fastest click from user: {}", random_int);
        random_int
    }
}