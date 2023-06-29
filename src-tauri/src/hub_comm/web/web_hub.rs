use rgb::RGB8;

use crate::hub_comm::common::hub_api::HubManager;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use crate::hub_comm::hw::internal::api_types::{TermButtonState, TermEvent};
use error_stack::Result;

#[derive(Debug, Default)]
pub struct WebHubManager {
    // ...
}

#[allow(dead_code, unused_variables)]
impl HubManager for WebHubManager {
    fn discover_terminals(&mut self) -> Result<Vec<u8>, HubManagerError> {
        todo!()
    }

    fn get_hub_timestamp(&self) -> Result<u32, HubManagerError> {
        todo!()
    }

    fn set_hub_timestamp(&self, _timestamp: u32) -> Result<(), HubManagerError> {
        todo!()
    }

    fn set_term_light_color(&self, _term_id: u8, _color: RGB8) -> Result<(), HubManagerError> {
        todo!()
    }

    fn set_term_feedback_led(
        &self,
        _term_id: u8,
        _state: &TermButtonState,
    ) -> Result<(), HubManagerError> {
        todo!()
    }

    fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError> {
        todo!()
    }
}
