use std::io::BufReader;
use std::collections::HashMap;
use std::fs::File;

use super::InstrDesc;

type OpMap = HashMap<String, InstrDesc>;

#[derive(Deserialize, Debug)]
pub struct CpuDesc {
    name: String,
    pub op_map: OpMap,
}

impl CpuDesc {
    pub fn new() -> Self {
        // TODO: proper file handling
        let cpu_desc =  File::open("cpu_desc/8080.json").unwrap();
        let reader = BufReader::new(cpu_desc);

        // TODO: error handling
        serde_json::from_reader(reader).unwrap()
    }
}
