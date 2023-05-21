use serde_xml_rs::from_str;
use std::fs;

use crate::game_pack::pack_dto::*;
use crate::game_pack::pack_entities::*;

fn parse_package(file_path: &str) -> PackageDto {
    let package_xml =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    from_str(&package_xml).unwrap()
}
