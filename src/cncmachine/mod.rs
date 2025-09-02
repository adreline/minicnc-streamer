use serialport::{SerialPortInfo, SerialPortType, available_ports};

pub mod cnc_machine {
    use std::time::Duration;

    pub struct CNCMachine {
        port_path: String,
        baud_rate: u32,
        head_state: bool,
    }
    impl Default for CNCMachine {
        fn default() -> Self {
            Self {
                port_path: "".to_string(),
                baud_rate: 0,
                head_state: false,
            }
        }
    }
    impl CNCMachine {
        pub(crate) fn new(port_path: &str, baud_rate: u32) -> Self {
            Self {
                port_path: port_path.to_string(),
                baud_rate,
                head_state: false,
            }
        }
        pub fn lift_the_head(&mut self) {
            let result = self.send_instruction("M300 S50");
            let Ok(_) = result else {
                return;
            };
            self.set_head_state(true);
        }
        pub fn descend_the_head(&mut self) {
            let result = self.send_instruction("M300 S30.00");
            let Ok(_) = result else {
                return;
            };
            self.set_head_state(false);
        }

        pub fn get_head_state(&self) -> bool {
            self.head_state
        }
        pub fn get_head_state_as_str(&self) -> &str {
            if self.get_head_state() {
                "Lifted"
            } else {
                "Descended"
            }
        }
        fn set_head_state(&mut self, state: bool) {
            self.head_state = state;
        }

        pub fn get_baud_rate(&self) -> u32 {
            self.baud_rate
        }
        pub fn get_port_path(&self) -> &str {
            self.port_path.as_str()
        }
        fn send_instruction(&self, instruction: &str) -> Result<(), Box<dyn std::error::Error>> {
            let port_path = self.port_path.clone();
            let baud_rate = self.baud_rate.clone();
            let mut port = serialport::new(port_path, baud_rate)
                .timeout(Duration::from_millis(10))
                .open()?;
            let mut write_buffer = instruction.as_bytes().to_vec();
            write_buffer.push(b'\r');
            let n = write_buffer.len(); // How many bytes to write to serial port.

            // Write to serial port
            port.write(&write_buffer[..n]) // blocks
                .unwrap();
            Ok(())
        }
    }
}

pub fn list_serial_ports() -> Vec<SerialPortInfo> {
    match available_ports() {
        Ok(mut ports) => {
            // Let's output ports in a stable order to facilitate comparing the output from
            // different runs (on different platforms, with different features, ...).
            ports.sort_by_key(|i| i.port_name.clone());
            let filtered_numbers: Vec<SerialPortInfo> = ports
                .iter()
                .filter(|&n| {
                    (n.port_type != SerialPortType::PciPort
                        && n.port_type != SerialPortType::Unknown)
                })
                .map(|n| n.clone())
                .collect();
            filtered_numbers
        }
        Err(e) => {
            let filtered_numbers: Vec<SerialPortInfo> = vec![];
            eprintln!("{:?}", e);
            eprintln!("Error listing serial ports");
            filtered_numbers
        }
    }
}
