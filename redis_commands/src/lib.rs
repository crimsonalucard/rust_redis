extern crate resp_parser;

use self::resp_parser::RespType;
use database::{Db, KeyValueStore};
use resp_parser::serialize_one_resp;
use resp_parser::RespType::{BulkString, RespError};
use std::sync::Arc;

enum Command<'a> {
    PING(&'a str),
    GET(&'a str),
    SET((&'a str, &'a RespType<'a>)),
}

fn resp_to_command<'a>(command: &'a RespType) -> Result<Command<'a>, &'static str> {
    match command {
        RespType::Array(elements) => {
            if elements.len() == 0 {
                Err("commands require an array length of one or more.")
            } else {
                let parameters = &elements[1..];
                match elements[0] {
                    BulkString(Some("PING")) => {
                        if parameters.len() == 0 {
                            Ok(Command::PING("PONG"))
                        } else if parameters.len() == 1 {
                            match parameters[0] {
                                BulkString(Some(ping_message)) => Ok(Command::PING(ping_message)),
                                _ => Err("invalid resp type in parameter for 'ping'"),
                            }
                        } else {
                            Err("wrong number of arguments for 'ping' command")
                        }
                    }
                    BulkString(Some("GET")) => {
                        if parameters.len() == 1 {
                            match parameters[0] {
                                BulkString(Some(key)) => Ok(Command::GET(key)),
                                _ => Err("invalid resp type for get"),
                            }
                        } else {
                            Err("wrong number of arguments for 'get' command")
                        }
                    }
                    BulkString(Some("SET")) => {
                        if parameters.len() == 2 {
                            let key = match parameters[0] {
                                BulkString(Some(key)) => key,
                                _ => {
                                    return Err("invalid key resp type for set");
                                }
                            };
                            let value = &parameters[1];
                            Ok(Command::SET((key, value)))
                        } else {
                            Err("wrong number of arguments for 'set' command")
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
fn execute_command(command: Result<Command, &str>, db: Arc<Db>) -> String {
    match command {
        Ok(Command::PING(arg)) => serialize_one_resp(&BulkString(Some(arg))),
        Ok(Command::GET(arg)) => db.get_value(arg),
        Ok(Command::SET((key, value))) => db.set_value(key, &value),
        Err(e) => serialize_one_resp(&RespError(("ERR", e))),
    }
}

pub fn handle_resp_token<'a>(resp_token: &'a mut RespType<'a>, db: Arc<Db>) -> String {
    execute_command(resp_to_command(resp_token), db)
}
