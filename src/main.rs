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
        if serial_read.len() > 13 {
            println!("Serial read: {:?}", serial_read);
            let pattern = &serial_read[0..12];
            let value = &serial_read[12..];

            match pattern.trim() {
                "44 48 48 55" => {
                    let humidity: u8 = u8::from_str_radix(value, 16).unwrap();
                    println!("Humidity: {}", humidity);
                }
                "44 48 54 45" => {
                    let temperature: i8 = i8::from_str_radix(value, 16).unwrap();
                    println!("Temperature: {}", temperature);
                }
                "4A 59 58 49" => {
                    if value.len() != 5 {
                        println!("Invalid pattern");
                        continue;
                    }
                    let pos_x_hex = &value.trim()[0..2];
                    let pos_y_hex = &value.trim()[3..5];
                    let pos_x: i8 = i8::from_str_radix(pos_x_hex, 16).unwrap();
                    let pos_y: i8 = i8::from_str_radix(pos_y_hex, 16).unwrap();

                    println!("Joystick: x:{} y:{}", pos_x, pos_y);
                }
                "52 46 49 44" => {
                    let tag_id = &value.trim();
                    println!("Tag ID: {}", tag_id);
                }
                _ => {
                    println!("Invalid pattern");
                }
            }

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
