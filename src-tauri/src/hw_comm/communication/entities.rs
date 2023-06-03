// Data definition
pub type DevId = u8;
pub type Timestamp = u32;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub enum TerminalButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct UartResponse {
    id: u8,
    status: ResponseStatus,
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
            payload,
        }
    }
}


