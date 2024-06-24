use serialport::new;
use serialport::SerialPortInfo;
use std::io;
use std::time::Duration;

fn main() {
    let ports = get_available_serial_ports();
    let mut input = String::new();

    println!("Listing available serial ports:");
    for port in ports {
        print!("Port: {} ", port.port_name);
        println!("Description: {:?}", port.port_type);
        println!("====================");
    }

    println!("Enter the port name you want to open:");
    io::stdin().read_line(&mut input).unwrap();
    let port_name = input.trim();

    let mut port = new(port_name, 9600).open().expect("Failed to open port");
    port.set_timeout(Duration::from_millis(100)).unwrap();

    let mut buf: Vec<u8> = vec![0; 1];
    let mut data = String::new();
    let mut serial_read = String::new();
    let match_start = "\n";
    let match_end = "\r";

    loop {
        let bytes = port.bytes_to_read().unwrap();
        if bytes > 0 {
            match port.read(buf.as_mut_slice()) {
                Ok(t) => {
                    // println!("Received {} bytes: {:?}", t, String::from_utf8_lossy(&buf[..t]));

                    let received_char = String::from_utf8_lossy(&buf[..t]);

                    if received_char == match_start {
                        data.clear();
                    } else if received_char == match_end {
                        // println!("data {}", data);
                        serial_read.clear();
                        serial_read.push_str(&data);
                    } else {
                        data.push_str(&received_char);
                    }
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
        if serial_read.len() > 0 {
            println!("{}", serial_read);
            serial_read.clear();
        }
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
