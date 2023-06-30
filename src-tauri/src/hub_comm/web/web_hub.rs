use std::str::FromStr;
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use rgb::RGB8;

use crate::hub_comm::common::hub_api::HubManager;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use crate::hub_comm::hw::internal::api_types::{TermButtonState, TermEvent};
use error_stack::{IntoReport, Report, Result, ResultExt};
use reqwest::Url;
use tokio::runtime::Runtime;
use crate::core::game_entities::{HubStatus, Player};
use crate::hub_comm::web::web_hub::internal_api::{TermFeedbackColor, TermLightColorDto, TimestampDto};
use crate::hub_comm::web::web_hub::internal_api::INTERNAL_API::*;
use crate::hub_comm::web::web_hub::server::PlayerIdentityDto;

mod server;
mod internal_api;
mod player_api;

const RETRY_INTERVAL_MS: u64 = 100;

#[derive(Debug)]
pub struct WebHubManager {
    pub base_url: Url,
    pub server_handle: Option<JoinHandle<()>>,
    pub client: reqwest::Client,
    pub rt: Runtime,
}

impl Default for WebHubManager {
    fn default() -> Self {
        let mut manager = Self {
            base_url: Url::from_str("http://localhost:8000/").expect("Bad base url"),
            server_handle: None,
            client: Default::default(),
            rt: Runtime::new().expect("No runtime - no game :D"),
        };
        manager.probe("").unwrap();
        manager
    }
}

fn start_hub_server() -> JoinHandle<()> {
    server::main();

    thread::spawn(move || {
        server::main();
    })
}

#[allow(dead_code, unused_variables)]
impl HubManager for WebHubManager {
    fn get_hub_address(&self) -> String {
        self.base_url.to_string()
    }

    fn probe(&mut self, _port: &str) -> Result<HubStatus, HubManagerError> {
        self.server_handle = Some(start_hub_server());
        for i in 0..5 {
            sleep(Duration::from_millis(RETRY_INTERVAL_MS));
            match self.get_hub_timestamp() {
                Ok(_) => return Ok(HubStatus::Detected),
                Err(err) => {
                    log::warn!("Can't reach web hub for {i} try. Err: {:?}", err);
                }
            }
        }

        log::error!("Web HUB can't be reached.");
        Err(Report::new(HubManagerError::HttpCommunicationError))
    }

    fn discover_players(&mut self) -> Result<Vec<Player>, HubManagerError> {
        let players: Vec<PlayerIdentityDto> = self.rt.block_on(async {
            self.client
                .get(self.base_url.join("players").expect("Bad URL join"))
                .send().await?
                .json().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;

        let players = players.iter()
            .map(|p| {
                let mut player = Player::default();
                player.term_id = p.id;
                player.name = p.name.clone();
                player
            })
            .collect();

        log::debug!("Received players: {:?}", players);
        Ok(players)
    }

    fn get_hub_timestamp(&self) -> Result<u32, HubManagerError> {
        let timestamp: TimestampDto = self.rt.block_on(async {
            self.client
                .get(self.base_url.join("timestamp").expect("Bad URL join"))
                .send().await?
                .json().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;

        log::debug!("Received players: {:?}", timestamp.timestamp);
        Ok(timestamp.timestamp)
    }

    fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError> {
        log::debug!("Setting timestamp of: {:?}", timestamp);

        self.rt.block_on(async {
            let dto = TimestampDto { timestamp };
            self.client
                .post(self.base_url.join("timestamp").expect("Bad URL join"))
                .json(&dto)
                .send().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;
        Ok(())
    }

    fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError> {
        log::debug!("Setting term {} color to {}", term_id, color);

        self.rt.block_on(async {
            let dto = TermLightColorDto {
                id: term_id,
                color: color.into(),
            };
            self.client
                .post(self.base_url.join("term-color").expect("Bad URL join"))
                .json(&dto)
                .send().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;
        Ok(())
    }

    fn set_term_feedback_led(&self, term_id: u8, state: &TermButtonState,
    ) -> Result<(), HubManagerError> {
        log::debug!("Setting feedback light for {} to {:?}", term_id, state);

        self.rt.block_on(async {
            let dto = TermFeedbackColor {
                id: term_id,
                state: state.to_bool(),
            };
            self.client
                .post(self.base_url.join("term-color").expect("Bad URL join"))
                .json(&dto)
                .send().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;
        Ok(())
    }

    fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError> {
        let events: Vec<TermEvent> = self.rt.block_on(async {
            self.client
                .get(self.base_url.join(TAKE_EVENT_QUEUE).expect("Bad URL join"))
                .send().await?
                .json().await
        }).into_report().change_context(HubManagerError::HttpCommunicationError)?;

        log::debug!("Received events: {:?}", events);
        Ok(events)
    }
}
