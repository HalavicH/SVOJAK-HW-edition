use std::io::{Read, Write};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver};
use std::thread;
use std::thread::JoinHandle;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serialport::SerialPort;
use crate::core::hub_manager::{HubRequest};
use crate::hw_comm::api::{HubIoError, ResponseStatus, HubResponse};
use crate::hw_comm::api::ProtocolVersion::Version;
use crate::hw_comm::uart_adapter::byte_handler::{ByteHandler, START_BYTE, STOP_BYTE};

#[derive(Debug)]
pub struct HubProtocolIoHandler {
    fsm_byte_handler: Arc<Mutex<ByteHandler>>,
    fsm_frame_rx: Receiver<Vec<u8>>,
    port_handle: Arc<Mutex<Box<dyn SerialPort>>>,
    listening_thread: Option<JoinHandle<()>>,
}

impl HubProtocolIoHandler {
    pub fn new(port_handle: Box<dyn SerialPort>) -> Self {
        let (_, fsm_frame_rx) = mpsc::channel::<Vec<u8>>();

        Self {
            port_handle: Arc::new(Mutex::new(port_handle)),
            fsm_byte_handler: Arc::new(Mutex::new(ByteHandler::new())),
            fsm_frame_rx,
            listening_thread: None,
        }
    }

    pub fn start_listening(&mut self) {
        let byte_handler_ptr = Arc::clone(&self.fsm_byte_handler);
        let port_handle_ptr = Arc::clone(&self.port_handle);

        let handle = thread::spawn(move || {
            loop {
                let mut byte: [u8; 1] = [0];
                let mut port_handle = port_handle_ptr.lock().unwrap();
                // Читає байт
                port_handle.read_exact(&mut byte).unwrap();

                let mut byte_handler = byte_handler_ptr.lock().unwrap();
                // Хендлить його. Як збереться фрейм byte_handler відправить його в fsm_frame_rx (64 строчка)
                byte_handler.handle_byte(byte[0]);
            }
        });

        self.listening_thread = Some(handle);
    }

    pub fn send_command(&self, request: HubRequest) -> Result<HubResponse, HubIoError> {
        let frame = assemble_frame(request.cmd(), request.payload());
        let stuffed_frame = stuff_bytes(&frame);

        {
            let mut port_handle = self.port_handle.lock()
                .map_err(|e| {
                    // Report::new(e).change_context(HubIoError::InternalError)
                    Report::new(HubIoError::InternalError)
                })?;

            port_handle.write_all(&stuffed_frame).into_report()
                .change_context(HubIoError::SerialPortError)?;
        }

        let response_frame = self.read_response_frame()?;

        let id = response_frame[1];
        let status = ResponseStatus::from(response_frame[2]);
        let payload = response_frame[3..].to_vec();

        Ok(HubResponse::new(id, status, payload))
    }

    fn read_response_frame(&self) -> Result<Vec<u8>, HubIoError> {
        let byte_handler_ptr = Arc::clone(&self.fsm_byte_handler);
        let port_handle_ptr = Arc::clone(&self.port_handle);
        let mut port_handle = port_handle_ptr.lock().unwrap();
        let mut byte_handler = byte_handler_ptr.lock().unwrap();
        let mut byte: [u8; 1] = [0];

        byte_handler.reset();

        while byte[0] != START_BYTE {
            port_handle.read_exact(&mut byte)
                .into_report().change_context(HubIoError::NoResponseFromHub)
                .attach_printable("Probably timeout")?;
        }
        // Handle start byte
        byte_handler.handle_byte(byte[0]);

        loop {
            port_handle.read_exact(&mut byte).unwrap();
            byte_handler.handle_byte(byte[0]);

            if byte[0] == STOP_BYTE {
                log::trace!("Finished frame reading");
                break;
            }
        }

        Ok(byte_handler.get_current_frame())
    }
}

impl Drop for HubProtocolIoHandler {
    fn drop(&mut self) {
        if let Some(handle) = self.listening_thread.take() {
            handle.join().unwrap();
        }
    }
}

pub fn format_bytes_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn stuff_bytes(frame: &Vec<u8>) -> Vec<u8> {
    let mut stuffed = vec![START_BYTE];
    for byte in frame {
        match *byte {
            0xC0..=0xCF => {
                stuffed.push(0xC1);
                stuffed.push(*byte & 0x0F);
            }
            _ => {
                stuffed.push(*byte)
            }
        }
    }
    stuffed.push(STOP_BYTE);
    log::debug!("Frame after bit stuffing: {:?}", format_bytes_hex(&stuffed));

    stuffed
}

pub fn assemble_frame(cmd: u8, mut payload: Vec<u8>) -> Vec<u8> {
    let payload_len = payload.len() as u8;
    let tid = 0;
    let mut frame = vec![Version.to_value(), tid, cmd, payload_len];
    frame.append(&mut payload);
    log::debug!("Assembled frame: {:?}", format_bytes_hex(&frame));
    frame
}

#[cfg(test)]
mod tests {
    use crate::hw_comm::api::ProtocolVersion::Version;
    use crate::hw_comm::uart_adapter::byte_handler::{START_BYTE, STOP_BYTE};
    use crate::hw_comm::uart_adapter::hub_protocol_io_handler::{assemble_frame, stuff_bytes};

    #[test]
    fn test_frame_assembly() {
        let expected = vec![
            START_BYTE,
            Version.to_value(),
            0x00,
            0x90,
            0x03,
            0x01, 0x02, 0x03,
            STOP_BYTE];
        let frame = assemble_frame(0x90, vec![0x01, 0x02, 0x03]);
        assert_eq!(frame, expected);
    }

    #[test]
    fn test_byte_stuffing_when_no_stuffing_occurs() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let goal = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let result = stuff_bytes(&input);
        assert_eq!(result, goal);
    }

    #[test]
    fn test_byte_stuffing() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0xC0, 0xC1, 0xCF];
        let goal = vec![0x03, 0x00, 0x90, 0x03, 0xC1, 0x00, 0xC1, 0x01, 0xC1, 0x0F];
        let result = stuff_bytes(&input);
        assert_eq!(result, goal);
    }
}

