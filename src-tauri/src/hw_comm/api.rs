use serde::Serialize;
use std::error::Error;
use std::fmt;

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

pub enum TerminalButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct UartResponse {
    id: u8,
    status: ResponseStatus,
    payload_len: u8,
    payload: Vec<u8>,
}

#[derive(Debug)]
pub enum ResponseStatus {
    Ok = 0x00,
    GenericError = 0x80,
    DeviceNotResponding = 0x90,
    UnknownError,
}

impl From<u8> for ResponseStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ResponseStatus::Ok,
            0x80 => ResponseStatus::GenericError,
            0x90 => ResponseStatus::DeviceNotResponding,
            _ => ResponseStatus::UnknownError,
        }
    }
}

impl UartResponse {
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

// struct HubConnection {
//
// }