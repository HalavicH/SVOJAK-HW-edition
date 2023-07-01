use crate::api::dto::QuestionType;
use crate::game_pack::translator::translate;
use serde::Serialize;
use std::collections::HashMap;

// Game entities
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum QuestionMediaType {
    Say,
    Voice,
    Video,
    Marker,
    Image,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub atom_type: QuestionMediaType,
    pub content: String,
}

impl Atom {
    pub fn translate(&mut self, lang: &str) -> &mut Atom {
        if self.atom_type == QuestionMediaType::Say {
            self.content = translate(self.content.as_str(), lang);
        }

        self
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Question {
    pub scenario: Vec<Atom>,
    pub right_answer: String,
    pub question_type: QuestionType,
    pub price: i32,
}

impl Question {
    pub fn translate(&mut self, lang: &str) -> &mut Question {
        self.right_answer = translate(self.right_answer.as_str(), lang);
        for atom in &mut self.scenario {
            atom.translate(lang);
        }

        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub name: String,
    pub questions: HashMap<i32, Question>,
}

impl Theme {
    pub fn pop_question(&mut self, price: &i32) -> Option<Question> {
        self.questions.remove(price)
    }

    pub fn get_question(&self, price: &i32) -> Option<&Question> {
        self.questions.get(price)
    }

    pub fn translate(&mut self, lang: &str) -> &mut Theme {
        self.name = translate(self.name.as_str(), lang);
        for question in &mut self.questions {
            question.1.translate(lang);
        }

        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Round {
    pub name: String,
    pub round_type: String,
    pub themes: HashMap<String, Theme>,
    pub question_count: i32,
    pub normal_question_count: i32,
    pub pip_question_count: i32,
    pub questions_left: i32,
}

impl Round {
    pub fn decrement_round(&mut self) {
        self.questions_left -= 1;
    }

    pub fn translate(&mut self, lang: &str) -> &mut Round {
        self.name = translate(self.name.as_str(), lang);
        let mut new_themes: HashMap<String, Theme> = Default::default();

        for theme in &mut self.themes {
            theme.1.translate(lang);
        }

        for theme in &self.themes {
            new_themes.insert(theme.1.name.clone(), theme.1.clone());
        }

        self.themes = new_themes.clone();

        self
    }
}

// Pack information
#[derive(Debug, PartialEq, Clone)]
pub struct Author {
    pub name: String,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Info {
    pub authors: Vec<Author>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PackContent {
    pub name: String,
    pub version: String,
    pub id: String,
    pub restriction: String,
    pub date: String,
    pub difficulty: u8,
    pub info: Info,
    pub rounds: Vec<Round>,
}

impl PackContent {
    pub fn translate(&mut self, lang: &str) -> &mut PackContent {
        self.name = translate(self.name.as_str(), lang);
        for round in &mut self.rounds {
            round.translate(lang);
        }

        self
    }
}
