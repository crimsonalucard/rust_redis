extern crate resp_parser;

use self::resp_parser::RespType;
use resp_parser::RespType::{BulkString, RespError};

enum Command<'a> {
    PING(&'a str),
}

fn resp_to_command<'a>(command: &'a RespType) -> Result<Command<'a>, &'static str> {
    match command {
        RespType::Array(elements) => {
            if elements.len() == 0 {
                Err("commands require an array length of one or more.")
            } else {
                let parameters = &elements[1..];
                match *(elements[0]) {
                    BulkString(Some("PING")) => {
                        if parameters.len() == 0 {
                            Ok(Command::PING("PONG"))
                        } else if parameters.len() == 1 {
                            match *parameters[0] {
                                BulkString(Some(ping_message)) => Ok(Command::PING(ping_message)),
                                _ => Err("invalid resp type in parameter for 'ping'"),
                            }
                        } else {
                            Err("wrong number of arguments for 'ping' command")
                        }
                    }
                    _ => Err("unsupported command"),
                }
            }
        }
        _ => Err("Invalid command. Command must be an array."),
    }
}

//execution of command should happen here.
fn execute_command<'a>(command: Result<Command<'a>, &'a str>) -> RespType<'a> {
    match command {
        Ok(Command::PING(arg)) => BulkString(Some(arg)),
        Err(e) => RespError(("ERR", e)),
    }
}

pub fn handle_resp_token<'a>(resp_token: &'a RespType<'a>) -> RespType<'a> {
    execute_command(resp_to_command(resp_token))
}
