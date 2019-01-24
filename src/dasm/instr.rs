use std::fmt;
use super::InstrDesc;

pub type Data = [u8; 2];

#[derive( Debug)]
pub struct Instr<'a> {
    pub code: u16,
    pub desc: &'a InstrDesc,
    pub data: Data,
}

impl<'a> Instr<'a> {
    pub fn unknown(code: u16, desc: &'a InstrDesc) -> Instr<'a> {
        Instr {
            code,
            desc: desc,
            data: [0, 0],
        }
    }

    pub fn size(&self) -> usize {
        self.desc.size
    }

    fn name(&self) -> &str {
        &self.desc.name
    }

    fn fmt_data(&self) -> String {
        match self.data {
            [0, 0] => String::from(""),
            data => format!("{:x}{:x}", data[0], data[1]), 
        }
    }
}

impl<'a> fmt::Display for Instr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:2x} | {} {}",
            self.code, self.name(), self.fmt_data()
        )
    }
}
