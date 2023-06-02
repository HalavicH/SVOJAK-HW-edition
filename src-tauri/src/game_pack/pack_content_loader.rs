use std::collections::HashMap;
use serde_xml_rs::from_str;
use std::fs;

use crate::game_pack::pack_content_dto::*;
use crate::game_pack::pack_content_entities::*;
use crate::game_pack::game_pack_entites::{PackLocationData};

pub fn load_pack_content(game_information: &PackLocationData) -> PackContent {
    let package_content_file_str = game_information.content_file_path.to_str().unwrap();
    let package: PackageDto = parse_package(package_content_file_str);

    map_package(package)
}
fn parse_package(file_path: &str) -> PackageDto {
    let package_xml =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    from_str(&package_xml).unwrap()
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
    };
    let vec = Vec::from_iter(round.themes.values());
    round.question_count = vec.iter()
        .map(|&theme| { theme.questions.len() as i32 })
        .sum::<i32>();

    round.questions_left = round.question_count;
    round
}
