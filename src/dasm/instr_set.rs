use super::{CpuDesc, Buffer, InstrDesc, InstrIterator, Instr};
use super::instr;

pub struct InstrSet {
    cpu_desc: CpuDesc,
    buffer: Buffer,
    null_desc: InstrDesc,
}

impl InstrSet {
    pub fn new() -> Self {
        Self {
            cpu_desc: CpuDesc::new(),
            buffer: Buffer::new(),
            null_desc: InstrDesc::unkwnown(),
        }
    }

    pub fn iter<'a>(&'a self) -> InstrIterator<'a> {
        InstrIterator::new(&self)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn next(&self, position: usize) -> Instr {
        let code = self.buffer.at(position) as u16;

        match self.instr_desc_for(code) {
            None => Instr::unknown(code, &self.null_desc),
            Some(instr_desc) => {
                Instr {
                    code,
                    desc: instr_desc,
                    data: self.data_for(instr_desc.size, position),
                }
            },
        }
    }

    fn instr_desc_for(&self, code: u16) -> Option<&InstrDesc> {
        self.cpu_desc.op_map.get(&format!("{:x}", code)) 
    }

    // TODO: universal + positioning
    fn data_for(&self, count: usize, position: usize) -> instr::Data {
        match count {
            2 => [self.buffer.at(position + 1), 0],
            3 => [self.buffer.at(position + 1), self.buffer.at(position + 2)],
            _ => [0, 0]
        }
    }
}
