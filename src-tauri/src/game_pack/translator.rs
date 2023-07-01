use htmlentity::entity::{decode, ICodedDataTrait};
use reqwest;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::io::Read;

const API_KEY: &'static str = "AIzaSyDWx_T78XsuJslvgST0lJ5v_DYGe_Y-OeI";

pub fn translate(input: &str, language: &str) -> String {
    let url = format!(
        "https://translation.googleapis.com/language/translate/v2?q={}&target={}&key={}",
        input, language, API_KEY
    );

    let response = send_request(url.as_str());
    let parsed_text = get_translated_text(&response);

    parsed_text.unwrap()
}

pub fn determine_lang(input: &str) -> String {
    let url = format!(
        "https://translation.googleapis.com/language/translate/v2/detect?key={}",
        API_KEY
    );
    let mut map = HashMap::new();

    map.insert("q", input);

    // Create a new client object
    let client = reqwest::blocking::Client::new();
    let resp = match client.post(url).json(&map).send() {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    get_determined_text(resp.as_str())
}

fn get_determined_text(json_response: &str) -> String {
    // Parse the JSON response
    let parsed_json: Value = serde_json::from_str(json_response).unwrap();

    let language = parsed_json["data"]["detections"][0][0]["language"]
        .as_str()
        .unwrap_or("");

    String::from(language)
}

fn send_request(url: &str) -> String {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    resp
}

fn get_translated_text(json_response: &str) -> Option<String> {
    // Parse the JSON response
    let parsed_json: Result<Value> = serde_json::from_str(json_response);

    // Check if parsing was successful
    if let Ok(json) = parsed_json {
        // Access the "translations" array
        if let Some(translations) = json["data"]["translations"].as_array() {
            // Extract the first translation object
            if let Some(translation) = translations.get(0) {
                // Extract the "translatedText" field as a string
                if let Some(translated_text) = translation["translatedText"].as_str() {
                    // Return the value as a String
                    return Some(decode(translated_text.as_bytes()).to_string().unwrap());
                }
            }
        }
    }

    None
}
