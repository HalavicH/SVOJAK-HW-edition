use serde_xml_rs::from_str;
use std::fs;

use crate::game_pack::pack_dto::*;
use crate::game_pack::pack_entities::*;

fn parse_package(file_path: &str) -> PackageDto {
    let package_xml =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    from_str(&package_xml).unwrap()
}

fn map_package(dto: PackageDto) -> Package {
    let map_atoms = |a: &AtomDto| Atom {
        atom_type: {
            match a.r#type {
                AtomTypeDto::say => AtomType::Say,
                AtomTypeDto::voice => AtomType::Voice,
                AtomTypeDto::video => AtomType::Video,
                AtomTypeDto::marker => AtomType::Marker,
                AtomTypeDto::image => AtomType::Image,
            }
        },
        content: a.content.clone(),
    };

    let map_questions = |q: &QuestionDto| Question {
        price: q.price,
        scenario: {
            q.scenario
                .atoms_list
                .iter()
                .map(map_atoms)
                .collect::<Vec<Atom>>()
        },
        right_answer: q.right.answer.clone(),
    };

    let map_themes = |t: &ThemeDto| Theme {
        name: t.name.clone(),
        questions: {
            t.questions
                .questions_list
                .iter()
                .map(map_questions)
                .collect::<Vec<Question>>()
        },
    };

    let map_rounds = |r: &RoundDto| Round {
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
                .map(map_themes)
                .collect::<Vec<Theme>>()
        },
    };

    Package {
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
                .map(map_rounds)
                .collect::<Vec<Round>>()
        },
    }
}

pub fn load_pack(file_path: &str) -> Package {
    let package: PackageDto = parse_package(file_path);
    map_package(package)
}
