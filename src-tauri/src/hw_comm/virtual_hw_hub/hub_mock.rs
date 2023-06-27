use std::thread;
use std::time::Duration;
use std::cell::Cell;
use std::io::ErrorKind;
use std::thread::JoinHandle;
use error_stack::{IntoReport, ResultExt, Result, Report};
use serialport::{SerialPort, TTYPort};
use rand::prelude::*;
use std::collections::HashSet;
use rand::thread_rng;

use crate::hw_comm::api_types::{hub_frame_pos, ResponseStatus};
use crate::hw_comm::byte_handler::ByteHandler;

use crate::hw_comm::hub_protocol_io_handler::{format_bytes_hex, stuff_bytes};

pub fn run_hub_mock() -> Result<(Box<dyn SerialPort>, JoinHandle<()>), String> {
    let (host_handle, device_tty) = TTYPort::pair().expect("Unable to create ptty pair");
    let device_handle = serialport::new(device_tty.name().unwrap(), 0).open().unwrap();

    let mut hub_mock = HubMock::new(Box::new(host_handle));

    let handle = thread::spawn(move || {
        hub_mock.hub_mock_routine();
    });

    Ok((device_handle, handle))
}

#[derive(Debug)]
pub struct HubMock {
    port_handle: Box<dyn SerialPort>,
    terminals: Vec<u8>,
    byte_handler: ByteHandler,
    base_timestamp: u32,
}

impl HubMock {
    fn new(port_handle: Box<dyn SerialPort>) -> Self {
        Self {
            port_handle,
            terminals: generate_random_numbers(),
            byte_handler: ByteHandler::default(),
            base_timestamp: u32::default(),
        }
    }

    pub fn hub_mock_routine(&mut self) {
        loop {
            log::trace!("New reading attempt:");
            // Read data from the virtual port
            let mut buffer = [0; 1024];
            let bytes_read = match self.port_handle.read(&mut buffer) {
                Ok(val) => {
                    val
                }
                Err(err) => {
                    if err.kind() == ErrorKind::TimedOut {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    } else {
                        log::error!("Error in hub_mock_routine: {}", err);
                        break;
                    }
                }
            };

            log::debug!("Request: {}", format_bytes_hex(&buffer[..bytes_read]));
            let frame = buffer[..bytes_read].to_vec();

            let response_frame = self.process_request_frame(frame);
            let stuffed = stuff_bytes(&response_frame);

            log::debug!("Responding with: {}", format_bytes_hex(&stuffed));
            let bytes_written = self.port_handle.write(&stuffed)
                .unwrap();
        }
    }

    fn process_request_frame(&mut self, raw_frame: Vec<u8>) -> Vec<u8> {
        for byte in raw_frame {
            self.byte_handler.handle_byte(byte);
        }

        let input_frame = self.byte_handler.get_current_frame();

        if input_frame.len() < 4 {
            return vec![
                    0x03,
                    0x00,
                    0x90,
                    0x00,
                ];
        }

        let version = input_frame[hub_frame_pos::PROTOCOL_VERSION];
        let tid = input_frame[hub_frame_pos::TID];
        let cmd = input_frame[hub_frame_pos::COMMAND_OR_STATUS];
        let len = input_frame[hub_frame_pos::PAYLOAD_LEN];
        let payload = input_frame[hub_frame_pos::PAYLOAD..].to_vec();

        let result = self.process_cmd(cmd, payload);

        match result {
            Ok(response_payload) => {
                let mut response_frame = vec![
                    version,
                    tid,
                    0x00,
                    response_payload.len() as u8,
                ];
                response_frame.append(&mut response_payload.clone());
                response_frame
            }
            Err(err) => {
                vec![
                    version,
                    tid,
                    err.current_context().clone() as u8,
                    0x00,
                ]
            }
        }
    }

    fn process_cmd(&mut self, cmd: u8, payload: Vec<u8>) -> Result<Vec<u8>, ResponseStatus> {
        let mut response_payload = match cmd {
            0x80 => { // SetTimestamp
                self.base_timestamp = u32::from_le_bytes(payload.try_into()
                    .map_err(|_| {
                        Report::new(ResponseStatus::GenericError)
                    })?
                );
                vec![]
            }
            0x81 => { // GetTimestamp
                self.base_timestamp.to_le_bytes().to_vec()
            }
            0x82 => { // SetHubRadioChannel
                vec![]
            }
            0x83 => { // SetTermRadioChannel
                vec![]
            }
            0x90 => { // PingDevice
                let id = payload[0];
                if self.terminals.contains(&id) {
                    return Ok(vec![]);
                } else {
                    return Err(Report::new(ResponseStatus::GenericError));
                }
            }
            0x91 => { // SetLightColor
                vec![]
            }
            0x92 => { // SetFeedbackLed
                vec![]
            }
            0xA0 => { // ReadEventQueue
                vec![]
            }
            _ => panic!("Invalid command value {}", cmd),
        };
        Ok(response_payload)
    }
}

fn generate_random_numbers() -> Vec<u8> {
    let mut rng = thread_rng();
    let random_count = rng.gen_range(2..=10);
    let mut set = HashSet::new();
    let mut numbers = Vec::new();

    numbers.push(random_count as u8);

    while set.len() < random_count as usize {
        let num = rng.gen_range(1..=10) as u8;
        set.insert(num);
    }

    numbers.extend(set.into_iter());
    numbers
}
