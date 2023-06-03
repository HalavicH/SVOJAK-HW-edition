
// pub enum HubResponse {
//     Event(DevId, Timestamp, TerminalButtonState),
//     Status(HubStatus),
// }
//
// pub enum HubRequest {
//     SetTimestamp(Timestamp),
//     SetСolor(DevId, Color),
// }

// trait SvProtocol {
//     fn read_response() -> HubResponse;
//     fn send_request(request: HubRequest) -> HubStatus;
// }

use simplelog::*;

use std::fmt::{Display, Formatter};
use crate::communication::entities::{Color, DevId, TerminalButtonState, Timestamp};

pub enum Command {
    GetEvents = 0x01,
    SetColor = 0x02,
    SetTimestamp = 0x03,
    InvalidCommand, // TBD
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f {
            &mut _ => { write!(f, "test") }
        }
    }
}

