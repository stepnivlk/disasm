use std::io::BufReader;
use std::collections::HashMap;

use super::InstrDesc;

type OpMap = HashMap<String, InstrDesc>;

type AvailableCpus = [&'static str; 1];

const CPU_8080: &[u8] = include_bytes!("../../cpus/8080.json");
const CPU_8080_NAME: &'static str = "8080";
pub const AVAILABLE_CPUS: AvailableCpus = [&CPU_8080_NAME];

#[derive(Deserialize, Debug)]
pub struct CpuDesc {
    name: String,
    pub op_map: OpMap,
}

impl CpuDesc {
    pub fn new(cpu: &str) -> Self {
        let op_map = match cpu {
            "8080" => CPU_8080,
            _ => CPU_8080,
        };

        let reader = BufReader::new(op_map);

        // TODO: error handling
        serde_json::from_reader(reader).unwrap()
    }

    pub fn available() -> AvailableCpus {
        AVAILABLE_CPUS
    }
}
