use crate::resp_parser::RespType;

pub fn execute_command(command: RespType) -> std::result::Result<RespType, &'static str> {
    match command {
        RespType::Array(mut elements) => {
            if elements.len() == 0 {
                Err("commands require an array length of one or more.")
            } else {
                let command_type = *(elements.remove(0));
                let parameters = elements;
                handle_command(command_type, parameters)
            }
        },
        _ => {
            Err("Invalid command. Command must be an array.")
        }
    }
}

fn handle_command<'a>(command_type: RespType, paramters: Vec<Box<RespType<'a>>>) -> std::result::Result<RespType<'a>, &'static str> {
    match command_type {
        RespType::BulkString(optional_command_string) => {
            match optional_command_string {
                Some(command_string) => {
                    match command_string {
                        "PING" => {
                            Ok(RespType::SimpleString("PONG"))
                        }, _ => Err("command not recognized.")
                    }
                }, None => {
                    Err("first part of command cannot be a null")
                }
            }
        }, _ => Err("All commands and command parameters must be bulk strings")
    }
}