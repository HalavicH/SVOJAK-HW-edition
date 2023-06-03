use std::default::Default;
use std::error::Error;
use std::fmt;

use serde::Serialize;
use error_stack::{FutureExt, IntoReport, ResultExt, Result};
use rgb::{RGB, RGB8};
use serialport::SerialPort;
use crate::core::game_entities::HubStatus;

#[derive(Debug, Clone, Serialize)]
pub enum HubManagerError {
    SerialPortError,
    NoResponse,
    InternalError,
}

impl fmt::Display for HubManagerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to load game pack:")
    }
}

impl Error for HubManagerError {}

#[derive(Default, Debug)]
pub struct HubManager {
    pub port_name: String,
    pub port_handle: Option<Box<dyn SerialPort>>,
    pub status: HubStatus,
    pub radio_channel: i32,
    pub baudrate: i32,
    pub base_timestamp: u32,
}

// Life
impl HubManager {
    pub fn new(port: String, channel: i32, baudrate: i32, base_timestamp: u32) -> Self {
        Self { port_name: port, port_handle: Option::default(), status: HubStatus::NoDevice, radio_channel: channel, baudrate, base_timestamp }
    }
}

// API
impl HubManager {
    pub fn probe(&mut self, port: &str) -> Result<HubStatus, HubManagerError> {
        println!("Pretend hub discovery at: {port}");

        let port_handle = serialport::new(port, 9600)
            .open()
            .into_report()
            .change_context(HubManagerError::SerialPortError)
            .attach_printable(format!("Can't open port {port}"))?;

        // port_handle.baud_rate()
        self.port_name = port.to_owned();
        Ok(HubStatus::Detected)
    }

    // pub send_command()

    pub fn discover_terminals(&mut self, radio_channel: i32) -> Vec<u8> {
        println!("Pretend terminals discovery at: {radio_channel}");

        self.radio_channel = radio_channel;
        vec![1, 2, 3, 4]
    }

    pub fn set_timestamp() {
        println!("Pretend setting timestamp");
    }

    /// ### get current timestamp
    ///
    /// Command id: `0x81`
    ///
    /// #### response payload
    /// `[tid] [status] [response length] [response payload (timestamp)]`
    pub fn get_current_timestamp() -> u32 {
        println!("Pretend setting timestamp");
        100_100_100
    }
}

pub enum HubRequest {
    SetTimestamp(u32),
    GetTimestamp,
    SetRadioChannel(u8),
    PingDevice(u8),
    SetLightState(u8, RGB8),
    SetFeedbackLed(u8, bool),
    ReadEventQueue,
}

impl HubRequest {
    fn to_command(&self) -> u8 {
        match self {
            HubRequest::SetTimestamp(_) => 0x80,
            HubRequest::GetTimestamp => 0x81,
            HubRequest::SetRadioChannel(_) => 0x82,
            HubRequest::PingDevice(_) => 0x90,
            HubRequest::SetLightState(_, _) => 0x91,
            HubRequest::SetFeedbackLed(_, _) => 0x92,
            HubRequest::ReadEventQueue => 0xA0,
        }
    }
}
pub enum StatusCodes {
    Ok,                  // 0x00 command ok
    InternalError,       // 0x80 general error
    DeviceNotResponding, // 0x90 device is not responding (probably off or absent)
}