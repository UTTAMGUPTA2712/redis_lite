pub enum Command {
    Get,
    Set,
    Ping,
    Delete,
    Invalid,
}

impl Command {
    pub fn get_command(value: &str) -> Command {
        match value.as_bytes() {
            b"set" => Command::Set,
            b"get" => Command::Get,
            b"delete" => Command::Delete,
            b"ping" => Command::Ping,
            _ => Command::Invalid,
        }
    }
}
