use super::{InstrSet, Instr};

pub struct InstrIterator<'a> {
    data: &'a InstrSet,
    position: usize,
}

impl<'a> InstrIterator<'a> {
    pub fn new(data: &'a InstrSet) -> InstrIterator<'a> {
        Self {
            data,
            position: 0,
        }
    }
}

impl<'a> Iterator for InstrIterator<'a> {
    type Item = Instr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.data.len() {
            return None
        }

        let instr = self.data.next(self.position);
        self.position += instr.size();

        Some(instr)
    }
}
