use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::string::ParseError;

// Data definition
pub type DevId = u8;
pub type Timestamp = u32;

#[derive(Debug, Clone, Serialize)]
pub enum HubIoError {
    NotInitializedError,
    SerialPortError,
    NoResponseFromHub,
    InternalError,
}

impl fmt::Display for HubIoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to perform hub communication:")
    }
}

impl Error for HubIoError {}

#[derive(Debug)]
pub struct HubResponse {
    pub id: u8,
    pub status: ResponseStatus,
    pub payload_len: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ResponseStatus {
    Ok = 0x00,
    GenericError = 0x80,
    TerminalNotResponding = 0x90,
    UnknownError,
}

impl From<u8> for ResponseStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ResponseStatus::Ok,
            0x80 => ResponseStatus::GenericError,
            0x90 => ResponseStatus::TerminalNotResponding,
            _ => ResponseStatus::UnknownError,
        }
    }
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


/// Queries OS for all available serial ports
pub fn discover_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports()
        .expect("No ports found!");
    let mut ports_vec = Vec::new();

    log::info!("Serial ports: {:?}", ports);


    for p in ports {
        log::info!("{}", p.port_name);

        ports_vec.push(p.port_name.clone());
    }

    ports_vec
}

pub enum ProtocolVersion {
    V3 = 0x03,
}

impl ProtocolVersion {
    pub fn to_value(&self) -> u8 {
        0x03
    }
}

pub enum FramePositions {
    ProtocolVersion = 0,
    Tid = 1,
    Command = 2,
    PayloadLen = 3,
    Payload = 4,
}

pub struct TermEvent {
    pub term_id: u8,
    pub timestamp: u32,
    pub state: TermButtonState,
}

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
        fmt.write_str(&format!("Invalid TermButtonState value: {}", self))
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