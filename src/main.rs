#[macro_use]
extern crate clap;
use clap::App;
use disasm::{CpuDesc, Buffer, InstrSet};

fn main() {
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();

    if matches.is_present("list-cpus") {
        for cpu in CpuDesc::available().iter() {
            println!("{}", cpu);
        }

        return
    }

    // TODO: naming, consider moving init to `InstrSet`

    let bin_path = matches.value_of("bin").unwrap();
    let cpu = matches.value_of("cpu").unwrap();

    let cpu_desc = CpuDesc::new(cpu);
    let buffer = Buffer::new(bin_path);

    let instr_set = InstrSet::new(cpu_desc, buffer);

    for instr in instr_set.iter() {
        println!("{}", instr);
    }
}
