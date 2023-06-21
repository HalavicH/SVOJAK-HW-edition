use std::io::{Read, Write};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use error_stack::{Result};
use crate::core::hub_manager::HubRequest;
use crate::hw_comm::api::{HubIoError, ResponseStatus, UartResponse};
use crate::hw_comm::api::ProtocolVersion::V3;
use crate::hw_comm::uart_adapter::byte_handler::{ByteHandler, START_BYTE, STOP_BYTE};

const HUB_CMD_TIMEOUT: Duration = Duration::from_millis(100_000);

#[derive(Debug)]
pub struct HubProtocolIoHandler<Handle: Read + Write + ?Sized + Send + 'static> {
    fsm_byte_handler: Arc<Mutex<ByteHandler>>,
    fsm_frame_rx: Receiver<Vec<u8>>,
    port_handle: Arc<Mutex<Box<Handle>>>,
    listening_thread: Option<JoinHandle<()>>,
}

impl<Handle: Read + Write + ?Sized + Send> HubProtocolIoHandler<Handle> {
    pub fn new(port_handle: Box<Handle>) -> Self {
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
                // Читає байт
                port_handle.read_exact(&mut byte).unwrap();

                let mut byte_handler = byte_handler_ptr.lock().unwrap();
                // Хендлить його. Як збереться фрейм byte_handler відправить його в fsm_frame_rx (64 строчка)
                byte_handler.handle_byte(byte[0]);
            }
        });

        self.listening_thread = Some(handle);
    }

    // Accept HubRequest -> HubResponse. Convert internally
    pub fn send_command(&self, request: HubRequest) -> Result<UartResponse, HubIoError> {
        let frame = Self::assemble_frame(request.cmd(), request.payload());
        let stuffed_frame = Self::stuff_bytes(&frame);

        let mut port_handle = self.port_handle.lock().unwrap();
        port_handle.write_all(&stuffed_frame).unwrap();

        let curr_frame = self.fsm_frame_rx.recv_timeout(HUB_CMD_TIMEOUT).unwrap();

        let id = curr_frame[1];
        let status = ResponseStatus::from(curr_frame[2]);
        let payload = curr_frame[3..].to_vec();

        Ok(UartResponse::new(id, status, payload))
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
        let mut frame = vec![START_BYTE, V3.to_value(), tid, cmd, payload_len, STOP_BYTE];
        frame.append(&mut payload);
        log::debug!("Assembled frame: {:?}", format_bytes_hex(&frame));
        frame
    }
}

impl<Handle: Read + Write + ?Sized + Send> Drop for HubProtocolIoHandler<Handle> {
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
    use std::fs::{File, OpenOptions};
    use crate::core::hub_manager::HubRequest::GetTimestamp;
    use crate::hw_comm::uart_adapter::hub_protocol_io_handler::HubProtocolIoHandler;

    #[test]
    fn test_frame_assembly() {
        let result = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let frame = HubProtocolIoHandler::<File>::assemble_frame(0x90, vec![0x01, 0x02, 0x03]);
        assert_eq!(frame, result);
    }

    #[test]
    fn test_byte_stuffing_when_no_stuffing_occurs() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let goal = vec![0x03, 0x00, 0x90, 0x03, 0x01, 0x02, 0x03];
        let result = HubProtocolIoHandler::<File>::stuff_bytes(&input);
        assert_eq!(result, goal);
    }

    #[test]
    fn test_byte_stuffing() {
        let input = vec![0x03, 0x00, 0x90, 0x03, 0xC0, 0xC1, 0xCF];
        let goal = vec![0x03, 0x00, 0x90, 0x03, 0xC1, 0x00, 0xC1, 0x01, 0xC1, 0x0F];
        let result = HubProtocolIoHandler::<File>::stuff_bytes(&input);
        assert_eq!(result, goal);
    }

    #[test_log::test] // Automatically wraps test to initialize logging
    fn test_send_request() {
        println!("Works");
        let port_handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/ttys004")
            .expect("Failed to open virtual port");

        let mut hub: HubProtocolIoHandler<File> = HubProtocolIoHandler::new(Box::new(port_handle));
        hub.start_listening();
        let _ = hub.send_command(GetTimestamp);
    }
}

// send_request():
//     uart.write()  // <-- same uart handle
//     let msg_from_reader = rx.recv();
//
// reader_thread():
//     loop {
//         let byte = uart.read_byte(); // <-- same uart handle
//         if byte == END_BYTE:
//             tx.send(msg_from_reader)
//         else:
//             msg_from_reader.push(byte)
//     }
//     ...