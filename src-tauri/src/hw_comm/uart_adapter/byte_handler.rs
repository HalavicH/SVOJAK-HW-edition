pub type RawFrame = Vec<u8>;

#[derive(Debug)]
pub enum ByteHandlerState {
    Byte,
    Escape,
}

pub const START_BYTE: u8 = 0xC0;
pub const STOP_BYTE: u8 = 0xCF;
const ESCAPE_BYTE: u8 = 0xC1;
const ESCAPE_MASK: u8 = 0xC0;

#[derive(Debug)]
pub struct ByteHandler {
    state: ByteHandlerState,
    framebuf: RawFrame,
}

impl ByteHandler {
    pub fn new() -> Self {
        Self {
            state: ByteHandlerState::Byte,
            framebuf: vec!(),
        }
    }

    pub fn reset(&mut self) {
        self.state = ByteHandlerState::Byte;
        self.framebuf = vec![];
    }

    pub fn handle_byte(&mut self, byte: u8) {
        log::debug!("Received byte: {byte:#X}");
        match self.state {
            ByteHandlerState::Byte => {
                self.handle_clean_byte(byte);
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

    fn handle_clean_byte(&mut self, byte: u8) {
        match byte {
            ESCAPE_BYTE => {
                self.state = ByteHandlerState::Escape;
                log::debug!("Got escape byte. Set state: {:?}", self.state);
            }
            START_BYTE => {
                self.state = ByteHandlerState::Byte;
                log::debug!("Got start byte. Set state: {:?}", self.state);
                self.framebuf.clear();
            }
            STOP_BYTE => {
                self.state = ByteHandlerState::Byte;
                log::debug!("Got end byte. Set state: {:?}", self.state);
                log::debug!("Resulting frame: {:?}", self.framebuf);
            }
            _ => self.framebuf.push(byte),
        }
    }

    pub fn get_current_frame(&self) -> Vec<u8> {
        self.framebuf.clone()
    }
}
