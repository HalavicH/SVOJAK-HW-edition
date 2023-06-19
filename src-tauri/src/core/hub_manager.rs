#[allow(dead_code)]
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use error_stack::{IntoReport, ResultExt, Result, Report};
use rgb::{RGB8};
use serialport::{SerialPort};
use crate::core::game_entities::HubStatus;

#[derive(Debug, Clone, Serialize)]
pub enum HubManagerError {
    NotInitializedError,
    SerialPortError,
    NoResponseFromHub,
    InternalError,
}

impl fmt::Display for HubManagerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to perform hub communication:")
    }
}

impl Error for HubManagerError {}

#[derive(Debug)]
pub struct HubManager {
    pub port_name: String,
    pub port_handle: Option<Box<dyn SerialPort>>,
    pub baudrate: i32,
    pub radio_channel: i32,
    pub base_timestamp: u32,
    pub allow_answer_timestamp: u32,
    pub last_status: HubStatus,
}

// Life
impl Default for HubManager {
    fn default() -> Self {
        Self {
            port_name: String::default(),
            port_handle: None,
            last_status: HubStatus::NoDevice,
            radio_channel: 0,
            baudrate: 200_000,
            base_timestamp: 0,
            allow_answer_timestamp: 0,
        }
    }
}

// API
impl HubManager {
    pub fn probe(&mut self, port: &str) -> Result<HubStatus, HubManagerError> {
        if self.port_handle.is_some() {
            log::info!("Previous port handle found: {:?}", self.port_handle);
            self.port_handle = None;
        }

        log::info!("Try to discover hub at port: {port}");
        self.port_name = port.to_owned();

        let baud_rate = 200_000;
        let serial_port = serialport::new(port, baud_rate).open()
            .into_report()
            .change_context(HubManagerError::SerialPortError)
            .attach_printable(format!("Can't open port {port}"))?;

        self.port_handle = Some(serial_port);
        self.init_timestamp()?;

        let result = self.set_hub_timestamp()?;
        Ok(result)
    }

    pub fn is_alive(&self) -> bool {
        self.get_hub_timestamp().is_ok()
    }

    pub fn get_delta_from_timestamp(&self) -> Result<u32, HubManagerError> {
        Ok(get_epoch_ms()? - self.base_timestamp)
    }

    pub fn discover_terminals(&mut self, radio_channel: i32) -> Vec<u8> {
        log::info!("Pretend terminals discovery at: {radio_channel}");

        self.radio_channel = radio_channel;
        vec![1, 2, 3, 4]
    }

    /// ### get hub timestamp
    /// #### response payload
    /// `[tid] [status] [response length] [response payload (timestamp)]`
    pub fn get_hub_timestamp(&self) -> Result<u32, HubManagerError> {
        log::info!("Pretend getting timestamp");

        if self.last_status == HubStatus::Detected {
            Ok(100_100_100)
        } else {
            Err(Report::new(HubManagerError::NoResponseFromHub))
        }
    }

    fn set_hub_timestamp(&mut self) -> Result<HubStatus, HubManagerError> {
        log::info!("Setting timestamp of {}", self.base_timestamp);

        // let response = self.send_command(HubRequest::SetTimestamp(0).to_command(), vec![])?;
        Ok(HubStatus::Detected)
    }

    fn init_timestamp(&mut self) -> Result<(), HubManagerError> {
        Ok(self.base_timestamp = get_epoch_ms()?)
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
    #[allow(dead_code)]
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

pub fn get_epoch_ms() -> Result<u32, HubManagerError> {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .into_report()
        .attach_printable("Can't get unix time")
        .change_context(HubManagerError::InternalError)?;

    let milliseconds_since_base: u32 = since_the_epoch
        .as_secs()
        .checked_mul(1000)
        .and_then(|ms| {
            let stripped_ms = ms & 0xFFFFFFFF;
            stripped_ms.checked_add(u64::from(since_the_epoch.subsec_nanos()) / 1_000_000)
        })
        .and_then(|ms| ms.try_into().ok())
        .ok_or(HubManagerError::InternalError)
        .into_report()
        .attach_printable("Can't process UNIX time to timestamp")?;

    Ok(milliseconds_since_base)
}

#[cfg(test)]
mod tests {
    use std::ptr::null;
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_get_epoch_ms() {
        // Get the expected result manually
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();

        let expected_milliseconds_since_base: u32 = since_the_epoch
            .as_secs()
            .checked_mul(1000)
            .and_then(|ms| {
                let stripped_ms = ms & 0xFFFFFFFF;
                stripped_ms.checked_add(u64::from(since_the_epoch.subsec_nanos()) / 1_000_000)
            })
            .and_then(|ms| ms.try_into().ok())
            .unwrap();

        // Call the actual function
        let result = get_epoch_ms();

        // Check the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_milliseconds_since_base);
    }

    #[test]
    fn test_hub_timestamp_init() {
        let mut hub = HubManager::default();
        assert_eq!(hub.base_timestamp, 0);

        hub.init_timestamp().unwrap();
        assert_eq!(hub.base_timestamp, get_epoch_ms().unwrap());
    }

    #[test]
    fn test_event_time_offset() {
        let mut hub = HubManager::default();
        hub.init_timestamp().unwrap();
        assert_eq!(hub.base_timestamp, get_epoch_ms().unwrap());

        sleep(Duration::from_secs(1));
        let terminal_timestamp = get_epoch_ms().unwrap();

        let execution_offset = 5;
        assert_eq!(terminal_timestamp, hub.base_timestamp + 1000 + execution_offset);
    }

    #[test]
    fn test_probe() {
        let mut hub = HubManager::default();
        hub.probe("/dev/tty.Bluetooth-Incoming-Port").unwrap();
        assert_eq!(hub.base_timestamp, get_epoch_ms().unwrap());

        sleep(Duration::from_secs(1));
        let terminal_timestamp = get_epoch_ms().unwrap();

        let execution_offset = 5;
        assert_eq!(terminal_timestamp, hub.base_timestamp + 1000 + execution_offset);
    }
}