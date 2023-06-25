use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use serialport::{SerialPort, TTYPort};
use svoyak_tauri_app::core::hub_manager::HubRequest::*;
use svoyak_tauri_app::hw_comm::api::{HubResponse, ResponseStatus};
use svoyak_tauri_app::hw_comm::api::ProtocolVersion::Version;
use svoyak_tauri_app::hw_comm::uart_adapter::byte_handler::{ByteHandler, START_BYTE};
use svoyak_tauri_app::hw_comm::uart_adapter::hub_protocol_io_handler::{format_bytes_hex, HubProtocolIoHandler, stuff_bytes};

const MOCK_ID: u8 = 6;
const MOCK_STATUS: ResponseStatus = ResponseStatus::Ok;
const MOCK_TIMESTAMP: u32 = 0xDEAD_BEEF;
const MOCK_EVENTS: [u8; 18] = [
    0x01, 0xDE, 0xAD, 0xBE, 0xEF, 0x01,
    0x02, 0xDE, 0xAD, 0xBE, 0xEF, 0x01,
    0x03, 0xDE, 0xAD, 0xBE, 0xEF, 0x01,
];

#[test_log::test]
fn test_virtual_pipe_communication() {
    let (mut host_handle, mut device_handle) = prepare_ports();

    let message_from_host = "Writing from host to HUB";
    device_handle.write_all(message_from_host.as_bytes()).unwrap();
    let mut buffer = [0_u8; 1024];
    let result_from_host_len = host_handle.read(&mut buffer).unwrap();

    let vec_host = buffer[..result_from_host_len].to_vec();
    let result_from_host = std::str::from_utf8(&vec_host).unwrap();
    println!("Result from host: {:?}", result_from_host);
    assert_eq!(message_from_host, result_from_host);

    let message_from_hub = "Writing from HUB to host";
    host_handle.write_all(message_from_hub.as_bytes()).unwrap();
    let mut buffer = [0_u8; 1024];
    let result_from_hub_len = device_handle.read(&mut buffer).unwrap();

    let vec_hub = buffer[..result_from_hub_len].to_vec();
    let result_from_hub = std::str::from_utf8(&vec_hub).unwrap();
    println!("Result from HUB: {:?}", result_from_hub);
    assert_eq!(message_from_hub, result_from_hub);
}

#[test_log::test]
fn test_send_request_timeout() {
    let (_, device_handle) = prepare_ports();
    let hub_handler: HubProtocolIoHandler = HubProtocolIoHandler::new(device_handle);

    let result = hub_handler.send_command(GetTimestamp);
    assert!(result.is_err());
}

#[test_log::test]
fn test_get_timestamp_command() {
    let (host_handle, device_handle) = prepare_ports();
    let hub_handler: HubProtocolIoHandler = HubProtocolIoHandler::new(device_handle);
    start_hub_mock(Box::new(host_handle));

    let expected = HubResponse {
        id: MOCK_ID,
        status: MOCK_STATUS,
        payload_len: 4,
        payload: MOCK_TIMESTAMP.to_le_bytes().to_vec(),
    };

    let result = hub_handler.send_command(GetTimestamp);
    println!("Result {:#?}", result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

//////// Helpers /////////
fn prepare_ports() -> (TTYPort, Box<dyn SerialPort>) {
    let (host_handle, device_tty) = TTYPort::pair().expect("Unable to create ptty pair");
    println!("PTYs:");
    println!("\thost TTY: {:?}", host_handle);
    println!("\tHUB  TTY: {:?}", device_tty);

    let device_handle = serialport::new(device_tty.name().unwrap(), 0).open().unwrap();
    println!("HUB handle: {:?}", device_handle);
    (host_handle, device_handle)
}

fn start_hub_mock(port_handle: Box<dyn SerialPort>) {
    thread::spawn(move || {
        hub_mock_routine(port_handle);
    });
}

fn hub_mock_routine(mut port_handle: Box<dyn SerialPort>) {
    loop {
        // println!("New reading attempt:");
        // Read data from the virtual port
        let mut buffer = [0; 1024];
        let bytes_read = match port_handle.read(&mut buffer) {
            Ok(val) => { val }
            Err(err) => {
                println!("Error: {}", err);
                thread::sleep(Duration::from_millis(1000));
                continue;
            }
        };

        println!("Received {}", format_bytes_hex(&buffer[..bytes_read]));
        let frame = buffer[..bytes_read].to_vec();
        let response_frame = process_request_frame(frame);
        let stuffed = stuff_bytes(&response_frame);

        println!("Responding with: {:?}", format_bytes_hex(&stuffed));
        port_handle.write(&stuffed).unwrap();
    }
}

fn process_request_frame(raw_frame: Vec<u8>) -> Vec<u8> {
    let mut byte_handler = ByteHandler::new();

    for byte in raw_frame {
        byte_handler.handle_byte(byte);
    }

    let input_frame = byte_handler.get_current_frame();
    let version = input_frame[0];
    let tid = input_frame[1];
    let cmd = input_frame[2];
    // let len = input_frame[3];
    // let payload = input_frame[4..].to_vec();

    let mut response_payload = match cmd {
        0x80 => vec![], // SetTimestamp
        0x81 => MOCK_TIMESTAMP.to_le_bytes().to_vec(), // GetTimestamp
        0x82 => vec![], // SetHubRadioChannel
        0x83 => vec![], // SetTermRadioChannel
        0x90 => vec![], // PingDevice
        0x91 => vec![], // SetLightColor
        0x92 => vec![], // SetFeedbackLed
        0xA0 => MOCK_EVENTS.to_vec(), // ReadEventQueue
        _ => panic!("Invalid command value {}", cmd),
    };

    let mut response_frame = vec![
        version,
        tid,
        0x00,
        response_payload.len() as u8,
    ];

    response_frame.append(&mut response_payload);
    response_frame
}

#[allow(non_snake_case)]
mod test {
    use svoyak_tauri_app::hw_comm::api::ProtocolVersion::Version;
    use svoyak_tauri_app::hw_comm::uart_adapter::byte_handler::START_BYTE;
    use crate::{MOCK_EVENTS, MOCK_TIMESTAMP, process_request_frame};

    #[test_log::test]
    fn test__process_request_frame__when__set_timestamp__then__ok() {
        let input = [0x03_u8,0x00, 0x80, 0x04, 0x01, 0x02, 0x03, 0x04].to_vec();
        let expected = [0x03_u8, 0x00, 0x00, 0x00].to_vec();

        let result = process_request_frame(input);
        assert_eq!(expected, result);
    }

    #[test_log::test]
    fn test__process_request_frame__when__get_timestamp__then__ok() {
        let input = [0x03_u8, 0x00, 0x81, 0x00].to_vec();
        let mut expected = [0x03_u8, 0x00, 0x00, 0x04].to_vec();
        expected.append(&mut MOCK_TIMESTAMP.to_le_bytes().to_vec());

        let result = process_request_frame(input);
        assert_eq!(expected, result);
    }

    #[test_log::test]
    fn test__process_request_frame__when__get_events__then__ok() {
        let input = [0x03_u8,0x00, 0xA0, 0x00].to_vec();
        let mut expected = [0x03_u8, 0x00, 0x00, MOCK_EVENTS.len() as u8].to_vec();
        expected.append(&mut MOCK_EVENTS.to_vec());

        let result = process_request_frame(input);
        assert_eq!(expected, result);
    }
}