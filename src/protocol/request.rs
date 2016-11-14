

enum Command {
    //Storage commands
    Set,
    ADD,
    REPLACE,
    APPEND,
    PREPEND,
    CAS,

    // Retrieve commands
    GET,
    GETS,
}

pub enum ParseError {
    UnknownCommand,
}

pub struct Request {
    cmd: Command,
}

pub fn parse(buff :Vec<u8>) -> Result<Request, ParseError> {
    Ok(Request{cmd: Command::ADD})
}
