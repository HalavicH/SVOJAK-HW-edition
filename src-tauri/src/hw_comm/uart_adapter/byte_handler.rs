use std::sync::mpsc::Sender;

pub type RawFrame = Vec<u8>;

#[derive(Debug)]
pub enum ByteHandlerState {
    Byte,
    Escape,
}

pub const START_BYTE: u8 = 0xC0;
pub const END_BYTE: u8 = 0xCF;
const ESCAPE_BYTE: u8 = 0xC1;
const ESCAPE_MASK: u8 = 0xC0;

pub struct ByteHandler {
    state: ByteHandlerState,
    framebuf: RawFrame,
    fsm_frame_tx: Sender<RawFrame>,
}

impl ByteHandler {
    pub fn new(fsm_frame_tx: Sender<RawFrame>) -> Self {
        Self {
            state: ByteHandlerState::Byte,
            framebuf: vec!(),
            fsm_frame_tx,
        }
    }
    pub fn handle_byte(&mut self, byte: u8) {
        log::debug!("Received byte: {byte:#X}");
        match self.state {
            ByteHandlerState::Byte => {
                match byte {
                    ESCAPE_BYTE => {
                        self.state = ByteHandlerState::Escape;
                        log::debug!("Got escape byte. Set state: {:?}", self.state);
                    }
                    START_BYTE => {
                        self.state = ByteHandlerState::Byte;
                        log::debug!("Got start byte. Set state: {:?}", self.state);
                    }
                    END_BYTE => {
                        self.state = ByteHandlerState::Byte;
                        log::debug!("Got end byte. Set state: {:?}", self.state);

                        self.send_uart_frame();
                        self.framebuf.clear();
                    }
                    _ => self.framebuf.push(byte),
                }
            }
            ByteHandlerState::Escape => {
                self.state = ByteHandlerState::Byte;
                log::debug!("!!! During escape state. Set state: {:?}", self.state);
                let original_byte = byte | ESCAPE_MASK;
                log::debug!("Recovered byte: {byte}");
                self.framebuf.push(original_byte);
            }
        }
    }

    fn send_uart_frame(&mut self) {
        log::debug!("Sending frame: {:?}", self.framebuf);
        let _ = self.fsm_frame_tx.send(self.framebuf.clone());
    }
}
