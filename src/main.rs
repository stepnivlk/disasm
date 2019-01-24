use disasm::InstrSet;

fn main() {
    let instr_set = InstrSet::new();
  
    for (_, instr) in instr_set.iter().enumerate() {
        println!("{}", instr);
    }

}
