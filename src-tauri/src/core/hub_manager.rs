use crate::core::game_entities::{HubManager, HubStatus};

impl HubManager {
    pub fn probe(&mut self, port: &String) -> HubStatus {
        println!("Pretend hub discovery at: {port}");

        self.port = port.clone();
        HubStatus::Detected
    }

    pub fn discover_terminals(&mut self, radio_channel: i32) -> Vec<u8> {
        println!("Pretend terminals discovery at: {radio_channel}");

        self.radio_channel = radio_channel;
        vec![1, 2, 3, 4]
    }
}
