use std::fs::File;
use std::io::Read;

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        // TODO: proper file handling
        let mut file = File::open("data/invaders.h").unwrap();
        let mut data = Vec::new();

        // TODO: error handling
        file.read_to_end(&mut data).unwrap();

        Self {
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn at(&self, loc: usize) -> u8 {
        if self.data.len() == loc {
            println!("invalid {}", loc);
            0
        } else {
            self.data[loc]
        }
    }
}
