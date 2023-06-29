use rgb::RGB8;
use crate::core::game_entities::HubStatus;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use crate::hub_comm::hw::internal::api_types::{TermButtonState, TermEvent};
use error_stack::{IntoReport, ResultExt, Result, Report};

pub trait HubManager {
    // Common
    fn discover_terminals(&mut self) -> Result<Vec<u8>, HubManagerError>;
    fn get_hub_timestamp(&self) -> Result<u32, HubManagerError>;
    fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError>;
    fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError>;
    fn set_term_feedback_led(&self, term_id: u8, state: &TermButtonState) -> Result<(), HubManagerError>;
    fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError>;

    fn probe(&mut self, port: &str) -> Result<HubStatus, HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn setup_hub_connection(&mut self, port: &str) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn set_hub_radio_channel(&self, channel_num: u8) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn set_term_radio_channel(&self, term_id: u8, channel_num: u8) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
    fn ping_terminal(&self, term_id: u8) -> Result<(), HubManagerError> {
        Err(Report::new(HubManagerError::ApiNotSupported))
    }
}