#[allow(dead_code, unused, unused_imports)]
use std::{collections::HashMap, io, fmt, error::Error, fs};

use error_stack::{IntoReport, Result, ResultExt};
use serde_xml_rs::from_str;
use crate::api::dto::QuestionType;

use crate::game_pack::pack_content_dto::*;
use crate::game_pack::pack_content_entities::*;
use crate::game_pack::game_pack_entites::{PackLocationData};

#[derive(Debug)]
pub struct ParsePackContentError;

impl fmt::Display for ParsePackContentError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to parse content: invalid pack content")
    }
}

impl Error for ParsePackContentError {}

pub fn load_pack_content(game_information: &PackLocationData) -> Result<PackContent, ParsePackContentError> {
    let package_content_file_str = game_information.content_file_path.to_str().unwrap();
    let package: PackageDto = parse_package(package_content_file_str)
        .attach_printable_lazy(|| {
            format!("Can't load pack content: parsing failed")
        })?;

    let content = map_package(package);
    Ok(content)
}

fn parse_package(file_path: &str) -> Result<PackageDto, ParsePackContentError> {
    let package_xml = fs::read_to_string(file_path)
        .into_report()
        .attach_printable_lazy(|| {
            format!("Can't open package content file: '{file_path}'")
        })
        .change_context(ParsePackContentError)?;

    let package_dto = from_str(&package_xml)
        .into_report()
        .attach_printable_lazy(|| {
            format!("Can't parse pack content XML file: '{file_path}'")
        })
        .change_context(ParsePackContentError)?;

    Ok(package_dto)
}

fn map_package(dto: PackageDto) -> PackContent {
    PackContent {
        name: dto.name,
        version: dto.version,
        id: dto.id,
        restriction: dto.restriction,
        date: dto.date,
        difficulty: dto.difficulty,
        info: Info {
            authors: {
                dto.info
                    .authors
                    .iter()
                    .map(|a| Author {
                        name: a.name.clone(),
                    })
                    .collect::<Vec<Author>>()
            },
        },
        rounds: {
            dto.rounds
                .rounds_list
                .iter()
                .map(map_round)
                .collect::<Vec<Round>>()
        },
    }
}

fn map_atoms(a: &AtomDto) -> Atom {
    Atom {
        atom_type: {
            match a.r#type {
                AtomTypeDto::say => QuestionMediaType::Say,
                AtomTypeDto::voice => QuestionMediaType::Voice,
                AtomTypeDto::video => QuestionMediaType::Video,
                AtomTypeDto::marker => QuestionMediaType::Marker,
                AtomTypeDto::image => QuestionMediaType::Image,
            }
        },
        content: a.content.clone(),
    }
}

fn map_question(q: &QuestionDto) -> Question {
    Question {
        price: q.price,
        scenario: {
            q.scenario
                .atoms_list
                .iter()
                .map(map_atoms)
                .collect::<Vec<Atom>>()
        },
        right_answer: q.right.answer.clone(),
        // TODO: Set random pip
        question_type: QuestionType::Normal,
    }
}

fn map_theme(t: &ThemeDto) -> (String, Theme) {
    (
        t.name.clone(),
        Theme {
            name: t.name.clone(),
            questions: {
                t.questions
                    .questions_list
                    .iter()
                    .map(|q| (q.price, { map_question(q) }))
                    .collect::<HashMap<i32, Question>>()
            },
        }
    )
}

fn map_round(r: &RoundDto) -> Round {
    let mut round = Round {
        name: r.name.clone(),
        round_type: {
            match r.r#type {
                RoundTypeDto::normal => RoundType::Normal,
                RoundTypeDto::r#final => RoundType::Final,
            }
        },
        themes: {
            r.themes
                .themes_list
                .iter()
                .map(map_theme)
                .collect::<HashMap<String, Theme>>()
        },
        questions_left: -1,
        question_count: -1,
        normal_question_count: -1,
        pip_question_count: -1,
    };
    let vec = Vec::from_iter(round.themes.values());
    round.question_count = vec.iter()
        .map(|&theme| { theme.questions.len() as i32 })
        .sum::<i32>();

    round.questions_left = round.question_count;
    round.normal_question_count = round.question_count;
    round.pip_question_count = 0;
    round
}
