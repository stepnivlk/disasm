#[derive(Deserialize, Debug)]
pub struct InstrDesc {
    pub name: String,
    pub args: String,
    pub flags: String,
    pub function: String,
    pub size: usize,
}

impl InstrDesc {
    pub fn unkwnown() -> Self {
        Self {
            name: String::from("UNKNOWN"),
            args: String::from(""),
            flags: String::from(""),
            function: String::from(""),
            size: 1,
        }
    }
}
