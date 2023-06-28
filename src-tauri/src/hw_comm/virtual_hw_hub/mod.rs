use std::thread::JoinHandle;
use serialport::SerialPort;
use error_stack::{Result, Report};

use crate::core::hub_manager::HubManagerError;
#[cfg(unix)]
use crate::hw_comm::virtual_hw_hub::hub_mock::run_hub_mock;

#[cfg(unix)]
mod hub_mock;

pub const VIRTUAL_HUB_PORT: &str = "Demo HUB port";

#[cfg(unix)]
pub fn setup_virtual_hub_connection() -> Result<(Box<dyn SerialPort>, JoinHandle<()>), HubManagerError> {
    run_hub_mock().map_err(|_| {
        Report::new(HubManagerError::InternalError)
            .attach_printable("Can't create virtual hub.")
    })
}

#[cfg(not(unix))]
pub fn setup_virtual_hub_connection() -> Result<(Box<dyn SerialPort>, JoinHandle<()>), HubManagerError> {
    Err(Report::new(HubManagerError::InternalError)
            .attach_printable("Demo hub is not supported on Windows yet")
    )
}
