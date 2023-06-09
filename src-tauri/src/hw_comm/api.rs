/// Queries OS for all available serial ports
pub fn discover_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports()
        .expect("No ports found!");
    let mut ports_vec = Vec::new();

    log::info!("Serial ports: {:?}", ports);


    for p in ports {
        log::info!("{}", p.port_name);

        ports_vec.push(p.port_name.clone());
    }

    ports_vec
}
