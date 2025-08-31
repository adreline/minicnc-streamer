// Replacement values for gcode directives
pub static OPENBUILDS_COMPAT: [(&str, &str); 3] = [
    ("G0 Z5", "M300 S50"),    // Lift Pen; Tool Off
    (" F1000", ""),           // Unused
    ("G0 Z0", "M300 S30.00"), // Activate Pen; Tool On
];

pub mod gcode_loader {
    use crate::gcode::OPENBUILDS_COMPAT;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    pub struct GCodeLoader {
        pub filepath: PathBuf,
        pub gcode: Vec<String>,
    }
    impl Default for GCodeLoader {
        fn default() -> Self {
            Self { filepath: "".to_string().parse().unwrap(), gcode: vec![] }
        }
    }
    impl GCodeLoader {
        pub fn new(filepath: PathBuf) -> Self {
            let gcode = translate_gcode(load_gcode(filepath.clone()));
            Self { filepath, gcode }
        }
    }

    fn load_gcode(filepath: PathBuf) -> Vec<String> {
        read_to_string(filepath)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect() // gather them together into a vector
    }

    fn translate_gcode(gcode: Vec<String>) -> Vec<String> {
        let mut buffer: Vec<String> = vec![];
        for line in gcode.iter() {
            let mut temp_line = line.to_string();
            for pattern in &OPENBUILDS_COMPAT {
                temp_line = temp_line.replace(pattern.0, pattern.1);
            }
            buffer.push(temp_line);
        }
        buffer
    }
}
