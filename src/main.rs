use serialport::SerialPortInfo;

fn main() {
    let ports = get_available_serial_ports();
    for port in ports {
        print!("Port: {} ", port.port_name);
        println!("Description: {:?}", port.port_type);
        println!("====================");
    }
}

fn get_available_serial_ports() -> Vec<SerialPortInfo> {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                eprintln!("No serial ports found");
                vec![]
            } else {
                ports
            }
        }
        Err(e) => {
            eprintln!("Error listing serial ports: {:?}", e);
            vec![]
        }
    }
}
