use serde::Serialize;
use std::error::Error;
use std::fmt;
use rgb::RGB8;

#[derive(Debug, Clone, Serialize)]
pub enum HubIoError {
    NotInitializedError,
    SerialPortError,
    NoResponseFromHub,
    CorruptedResponseFromHub,
    InternalError,
}

impl fmt::Display for HubIoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to perform hub communication:")
    }
}

impl Error for HubIoError {}

/// HUB REQUEST
pub enum HubRequest {
    SetTimestamp(u32),
    GetTimestamp,
    SetHubRadioChannel(u8),
    SetTermRadioChannel(u8, u8),
    PingDevice(u8),
    SetLightColor(u8, RGB8),
    SetFeedbackLed(u8, bool),
    ReadEventQueue,
}

impl HubRequest {
    pub fn cmd(&self) -> u8 {
        match self {
            HubRequest::SetTimestamp(_) => 0x80,
            HubRequest::GetTimestamp => 0x81,
            HubRequest::SetHubRadioChannel(_) => 0x82,
            HubRequest::SetTermRadioChannel(_, _) => 0x82,
            HubRequest::PingDevice(_) => 0x90,
            HubRequest::SetLightColor(_, _) => 0x91,
            HubRequest::SetFeedbackLed(_, _) => 0x92,
            HubRequest::ReadEventQueue => 0xA0,
        }
    }

    pub fn payload(&self) -> Vec<u8> {
        match self {
            HubRequest::SetTimestamp(timestamp) => timestamp.to_be_bytes().to_vec(),
            HubRequest::GetTimestamp => vec![],
            HubRequest::SetHubRadioChannel(channel_num) => vec![*channel_num],
            HubRequest::SetTermRadioChannel(term_id, channel_num) => vec![*term_id, *channel_num],
            HubRequest::PingDevice(term_id) => vec![*term_id],
            HubRequest::SetLightColor(term_id, color) => vec![*term_id, color.r, color.g, color.b],
            HubRequest::SetFeedbackLed(term_id, state) => vec![*term_id, *state as u8],
            HubRequest::ReadEventQueue => vec![],
        }
    }
}

/// HUB RESPONSE
#[derive(Debug, Eq, PartialEq)]
pub struct HubResponse {
    pub id: u8,
    pub status: ResponseStatus,
    pub payload_len: u8,
    pub payload: Vec<u8>,
}

impl HubResponse {
    pub fn new(id: u8, status: ResponseStatus, payload: Vec<u8>) -> Self {
        Self {
            id,
            status,
            payload_len: payload.len() as u8,
            payload,
        }
    }
}

/// HUB RESPONSE STATUS
#[derive(Debug, Eq, PartialEq)]
pub enum ResponseStatus {
    Ok = 0x00,
    GenericError = 0x80,
    TerminalNotResponding = 0x90,
    UnknownError = 0xFF,
}

impl From<u8> for ResponseStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ResponseStatus::Ok,
            0x80 => ResponseStatus::GenericError,
            0x90 => ResponseStatus::TerminalNotResponding,
            | _ => ResponseStatus::UnknownError,
        }
    }
}

/// HUB PROTOCOL VERSION
pub enum ProtocolVersion {
    Version = 0x03,
}

impl ProtocolVersion {
    pub fn to_value(&self) -> u8 {
        0x03
    }
}

/// HUB FRAME ELEMENTS POSITION
pub mod hub_frame_pos {
    pub const PROTOCOL_VERSION: usize = 0;
    pub const TID: usize = 1;
    pub const COMMAND_OR_STATUS: usize = 2;
    pub const PAYLOAD_LEN: usize = 3;
    pub const PAYLOAD: usize = 4;
}

pub struct TermEvent {
    pub term_id: u8,
    pub timestamp: u32,
    pub state: TermButtonState,
}

/// Terminal button state enum
#[derive(Debug)]
pub enum TermButtonState {
    Pressed,
    Released,
}

impl TermButtonState {
    pub fn to_bool(&self) -> bool {
        match self {
            TermButtonState::Pressed => { true }
            TermButtonState::Released => { false }
        }
    }

    pub fn from_bool(state: bool) -> TermButtonState {
        match state {
            true => { TermButtonState::Pressed }
            false => { TermButtonState::Released }
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub enum MyParseError {
    FromU8(u8),
}

impl fmt::Display for MyParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Invalid TermButtonState value")
    }
}

impl Error for MyParseError {}

impl TryFrom<u8> for TermButtonState {
    type Error = MyParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TermButtonState::Released),
            1 => Ok(TermButtonState::Pressed),
            _ => Err(MyParseError::FromU8(value)),
        }
    }
}


