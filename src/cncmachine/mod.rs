use serialport::{available_ports, SerialPortInfo, SerialPortType};

pub mod cnc_machine {
    pub struct CNCMachine {
        //pub ports: Vec<SerialPortInfo>,
    }
    impl CNCMachine {
        pub(crate) fn new() -> Self {
            Self {  }
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
                .filter(|&n| (n.port_type != SerialPortType::PciPort && n.port_type != SerialPortType::Unknown  ))
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
