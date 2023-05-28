// Game entities
#[derive(Debug, PartialEq, Clone)]
pub enum AtomType {
    Say,
    Voice,
    Video,
    Marker,
    Image,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub atom_type: AtomType,
    pub content: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Question {
    pub scenario: Vec<Atom>,
    pub right_answer: String,
    pub price: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub name: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RoundType {
    Normal,
    Final,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Round {
    pub name: String,
    pub round_type: RoundType,
    pub themes: Vec<Theme>,
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
pub struct Package {
    pub name: String,
    pub version: String,
    pub id: String,
    pub restriction: String,
    pub date: String,
    pub difficulty: u8,
    pub info: Info,
    pub rounds: Vec<Round>,
}
