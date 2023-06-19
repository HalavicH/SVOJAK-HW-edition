use std::io::Read;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use error_stack::{ResultExt, Result, Report, IntoReport};
use serialport::SerialPort;
use crate::hw_comm::api::{HubIoError, ResponseStatus, UartResponse};
use crate::hw_comm::api::ProtocolVersion::V3;
use crate::hw_comm::uart_adapter::byte_handler::ByteHandler;

struct HubProtocolIoHandler {
    fsm_byte_handler: Arc<Mutex<ByteHandler>>,
    fsm_frame_rx: Receiver<Vec<u8>>,
    port_handle: Arc<Mutex<Box<dyn SerialPort>>>,
    listening_thread: Option<JoinHandle<()>>,
}

impl HubProtocolIoHandler {
    pub fn new(port_handle: Box<dyn SerialPort>) -> Self {
        let (fsm_frame_tx, fsm_frame_rx) = mpsc::channel::<Vec<u8>>();

        Self {
            port_handle: Arc::new(Mutex::new(port_handle)),
            fsm_byte_handler: Arc::new(Mutex::new(ByteHandler::new(fsm_frame_tx))),
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
                port_handle.read_exact(&mut byte).unwrap();

                let mut byte_handler = byte_handler_ptr.lock().unwrap();
                byte_handler.handle_byte(byte[0]);
            }
        });

        self.listening_thread = Some(handle);
    }

    pub fn send_command(&mut self, cmd: u8, payload: Vec<u8>) -> Result<UartResponse, HubIoError> {
        let frame = Self::assemble_frame(cmd, payload);
        let stuffed_frame = Self::stuff_bytes(&frame);

        let mut port_handle = self.port_handle.lock()
            .map_err(|_| {
                // Report::new(e.).change_context(HubIoError::InternalError)
                Report::new(HubIoError::InternalError)
            })?;
        port_handle.write_all(&stuffed_frame).into_report()
            .change_context(HubIoError::SerialPortError)?;

        let curr_frame: Vec<u8> = match self.fsm_frame_rx.recv() {
            Ok(x) => x,
            Err(_) => {
                log::warn!("Can't receive the frame");
                return Err(HubIoError::InternalError).into_report();
            },
        };

        let id = curr_frame[1];
        let status = ResponseStatus::from(curr_frame[2]);
        let payload = curr_frame[3..].to_vec();

        Ok(UartResponse::new(id,status, payload))
    }


    pub fn stuff_bytes(frame: &Vec<u8>) -> Vec<u8> {
        let mut stuffed = vec![];
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
        log::debug!("Frame after bit stuffing: {:?}", format_bytes_hex(&frame));

        stuffed
    }

    pub fn assemble_frame(cmd: u8, mut payload: Vec<u8>) -> Vec<u8> {
        let payload_len = payload.len() as u8;
        let tid = 0;
        let mut frame = vec![V3.to_value(), tid, cmd, payload_len];
        frame.append(&mut payload);
        log::debug!("Assembled frame: {:?}", format_bytes_hex(&frame));
        frame
    }
}

impl Drop for HubProtocolIoHandler {
    fn drop(&mut self) {
        if let Some(handle) = self.listening_thread.take() {
            handle.join().unwrap();
        }
    }
}

fn format_bytes_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::hw_comm::uart_adapter::hub_protocol_io_handler::HubProtocolIoHandler;

    #[test]
    fn test_frame_assembly() {

        let result = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let frame = HubProtocolIoHandler::assemble_frame(0x90, vec![0x01, 0x02, 0x03]);
        assert_eq!(frame, result);
    }

    #[test]
    fn test_byte_stuffing_when_no_stuffing_occurs() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let goal = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let result = HubProtocolIoHandler::stuff_bytes(&input);
        assert_eq!(result, goal);
    }

    #[test]
    fn test_byte_stuffing() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0xC0, 0xC1, 0xCF];
        let goal = vec![0x03, 0x00, 0x90, 0x03, 0xC1, 0x00, 0xC1, 0x01, 0xC1, 0x0F];
        let result = HubProtocolIoHandler::stuff_bytes(&input);
        assert_eq!(result, goal);
    }
}