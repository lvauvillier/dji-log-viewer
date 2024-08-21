pub struct FlightData {
    pub file_name: String,
    frames: Vec<dji_log_parser::frame::Frame>,
}

impl FlightData {
    pub fn new(file_name: String, frames: Vec<dji_log_parser::frame::Frame>) -> Self {
        Self { file_name, frames }
    }
}
