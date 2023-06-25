#[allow(dead_code)]
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use error_stack::{IntoReport, ResultExt, Result, Report};
use rgb::RGB8;
use serialport::{SerialPort};
use crate::core::game_entities::HubStatus;
use crate::hw_comm::api::{HubIoError, ResponseStatus, TermButtonState, TermEvent};
use crate::hw_comm::uart_adapter::hub_protocol_io_handler::HubProtocolIoHandler;

const HUB_CMD_TIMEOUT: Duration = Duration::from_millis(100_000);

#[derive(Debug, Clone, Serialize)]
pub enum HubManagerError {
    NotInitializedError,
    SerialPortError,
    NoResponseFromHub,
    NoResponseFromTerminal,
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
    pub hub_io_handler: Option<HubProtocolIoHandler>,
    pub baudrate: u32,
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
            hub_io_handler: None,
        }
    }
}

// API
impl HubManager {
    pub fn probe(&mut self, port: &str) -> Result<HubStatus, HubManagerError> {
        if self.hub_io_handler.is_some() {
            log::info!("Previous HUB io handle found: {:?}. Erasing", self.hub_io_handler.as_ref().unwrap());
            self.hub_io_handler = None;
        }

        log::info!("Try to discover hub at port: {port}");
        self.port_name = port.to_owned();

        let mut serial_port = serialport::new(port, self.baudrate).open()
            .into_report()
            .change_context(HubManagerError::SerialPortError)
            .attach_printable(format!("Can't open port {port}"))?;

        serial_port.set_timeout(HUB_CMD_TIMEOUT).into_report()
            .change_context(HubManagerError::InternalError)?;

        self.hub_io_handler = Some(HubProtocolIoHandler::new(serial_port));
        self.init_timestamp()?;
        self.set_hub_timestamp(self.base_timestamp)?;
        Ok(HubStatus::Detected)
    }

    pub fn is_hub_alive(&self) -> bool {
        self.get_hub_timestamp().is_ok()
    }

    pub fn get_delta_from_timestamp(&self) -> Result<u32, HubManagerError> {
        Ok(get_epoch_ms()? - self.base_timestamp)
    }

    pub fn discover_terminals(&mut self, radio_channel: i32) -> Result<Vec<u8>, HubManagerError> {
        log::info!("Pretend terminals discovery at: {radio_channel}");
        let mut terminals = vec![];

        self.radio_channel = radio_channel;
        self.set_hub_radio_channel(radio_channel as u8)?;

        // TODO: Check radio channel

        for term_id in 1..128 {
            if self.ping_terminal(term_id).is_ok() {
                log::debug!("Terminal #{} is alive", term_id);
                terminals.push(term_id);
            }
        }

        Ok(terminals)
    }

    /// ### get hub timestamp
    /// #### response payload
    /// `[tid] [status] [response length] [response payload (timestamp)]`
    pub fn get_hub_timestamp(&self) -> Result<u32, HubManagerError> {
        log::info!("Reading current HUB base timestamp");
        let handle = self.get_hub_handle_or_err()?;

        let response = handle.send_command(HubRequest::GetTimestamp)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        if response.status != ResponseStatus::Ok {
            return Err(Report::new(HubManagerError::InternalError));
        }

        let timestamp = parse_u32_from_vec(&response.payload)
            .change_context(HubManagerError::InternalError)
            .attach_printable(format!("Can't parse payload. {:?}", response.payload))?;

        log::info!("Got HUB timestamp: {}", timestamp);

        Ok(timestamp)
    }

    pub fn set_hub_timestamp(&self, timestamp: u32) -> Result<(), HubManagerError> {
        log::info!("Setting timestamp of 0x{:X?}", timestamp);
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::SetTimestamp(timestamp);
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    pub fn set_hub_radio_channel(&self, channel_num: u8) -> Result<(), HubManagerError> {
        log::info!("Setting hub radio channel to: {}", channel_num);
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::SetHubRadioChannel(channel_num);
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    pub fn set_term_radio_channel(&self, term_id: u8, channel_num: u8) -> Result<(), HubManagerError> {
        log::info!("Setting terminal radio channel to: {} for {}", channel_num, term_id);
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::SetTermRadioChannel(term_id, channel_num);
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    pub fn ping_terminal(&self, term_id: u8) -> Result<(), HubManagerError> {
        log::info!("Pinging terminal with id: #{}", term_id);
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::PingDevice(term_id);
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    pub fn set_term_light_color(&self, term_id: u8, color: RGB8) -> Result<(), HubManagerError> {
        log::info!("Setting terminal #{} light color to: {:?}", term_id, color);
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::SetLightColor(term_id, color);
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    pub fn set_term_feedback_led(&self, term_id: u8, state: &TermButtonState) -> Result<(), HubManagerError> {
        log::info!("Setting terminal #{} feedback light to: {:?}", term_id, state);
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::SetFeedbackLed(term_id, state.to_bool());
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)
    }

    pub fn read_event_queue(&self) -> Result<Vec<TermEvent>, HubManagerError> {
        log::info!("Reading event queue");
        let handle = self.get_hub_handle_or_err()?;

        let request = HubRequest::ReadEventQueue;
        let response = handle.send_command(request)
            .map_err(Self::hub_io_to_hub_mgr_error)?;

        map_status_to_result(response.status)?;

        let mut events = vec![];
        for chunk in response.payload.chunks_exact(std::mem::size_of::<TermEvent>()) {
            // Convert each chunk of bytes to a `TermEvent`
            let term_id = chunk[0];
            let timestamp = u32::from_ne_bytes(chunk[1..5].try_into().unwrap());
            let state_byte = chunk[5];
            let state = TermButtonState::try_from(state_byte)
                .into_report()
                .change_context(HubManagerError::InternalError)
                .attach_printable(format!("Can't parse TermButtonState for terminal {}", term_id))?;

            // Create a `TermEvent` struct
            let event = TermEvent {
                term_id,
                timestamp,
                state,
            };

            // Add the `TermEvent` to the events vector
            events.push(event);
        }

        Ok(events)
    }

    fn hub_io_to_hub_mgr_error(e: Report<HubIoError>) -> Report<HubManagerError> {
        match e.current_context() {
            HubIoError::NoResponseFromHub => {
                e.change_context(HubManagerError::NoResponseFromHub)
            }
            _ => { e.change_context(HubManagerError::InternalError) }
        }
    }

    fn init_timestamp(&mut self) -> Result<(), HubManagerError> {
        Ok(self.base_timestamp = get_epoch_ms()?)
    }

    fn get_hub_handle_or_err(&self) -> Result<&HubProtocolIoHandler, HubManagerError> {
        let connection = self.hub_io_handler.as_ref()
            .ok_or(HubManagerError::NotInitializedError)?;
        Ok(connection)
    }
}

// NotInitializedError,
// SerialPortError,
// NoResponseFromHub,
// InternalError,

fn map_status_to_result(status: ResponseStatus) -> Result<(), HubManagerError> {
    match status {
        ResponseStatus::Ok => {
            Ok(())
        }
        ResponseStatus::TerminalNotResponding => {
            Err(Report::new(HubManagerError::NoResponseFromTerminal))
        }
        _ => {
            Err(Report::new(HubManagerError::InternalError))
        }
    }
}

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

#[derive(Debug, Clone, Serialize)]
pub struct ParseU32Error {}

impl fmt::Display for ParseU32Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to parse u32 value:")
    }
}

impl Error for ParseU32Error {}


fn parse_u32_from_vec(vec: &Vec<u8>) -> Result<u32, ParseU32Error> {
    if vec.len() != 4 {
        return Err(ParseU32Error {}).into_report();
    }
    let mut result: u32 = 0;

    for &byte in vec {
        result = (result << 8) | byte as u32;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
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

        let execution_offset = 50;
        assert!(terminal_timestamp > hub.base_timestamp &&
            terminal_timestamp < (hub.base_timestamp + 1000 + execution_offset));
    }

    // #[test]
    // fn test_probe() {
    //     let mut hub = HubManager::default();
    //     hub.probe("/dev/tty.Bluetooth-Incoming-Port").unwrap();
    //     assert_eq!(hub.base_timestamp, get_epoch_ms().unwrap());
    //
    //     sleep(Duration::from_secs(1));
    //     let terminal_timestamp = get_epoch_ms().unwrap();
    //
    //     let execution_offset = 5;
    //     assert_eq!(terminal_timestamp, hub.base_timestamp + 1000 + execution_offset);
    // }
}