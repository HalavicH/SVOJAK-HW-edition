use simplelog::*;

use crate::uart_adapter::byte_handler::RawFrame;
use crate::communication::entities::{ResponseStatus, UartResponse};

const SIZE_PLACE: usize = 2;
const REQUIRED_ARGS_LEN: usize = 3;

pub fn handle_response_frame(curr_frame: RawFrame) {
    info!("Handling the frame {:?}", curr_frame);

    if (curr_frame.len() < REQUIRED_ARGS_LEN)
        || (curr_frame[SIZE_PLACE] != (curr_frame.len() - REQUIRED_ARGS_LEN) as u8) {
        warn!("Invalid frame received! Skipping.");
        return;
    }

    let id = curr_frame[0];
    let status = ResponseStatus::from(curr_frame[1]);
    let size = curr_frame[0];
    let payload: Vec<u8> = curr_frame[(SIZE_PLACE + 1)..curr_frame.len()].to_vec();

    let response = UartResponse::new(id, status, payload);

    info!("Parsed response: {:#?}", response);
}