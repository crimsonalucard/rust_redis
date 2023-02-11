use std::fmt;
use crate::resp_parser::RespType::{Array, BulkString, Integer, RespError, SimpleString};

pub enum RespType<'a> {
    SimpleString(&'a str),
    RespError((&'a str, &'a str)),
    Integer(i32),
    BulkString(Option<&'a str>),
    Array(std::vec::Vec<Box<RespType<'a>>>),
}

impl fmt::Display for RespType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleString(string) => write!(f, "SimpleString(\"{}\")", string),
            RespError((error_type, error_message)) => write!(f, "Error({}, {})", error_type, error_message),
            Integer(integer) => write!(f, "Integer({})", integer),
            BulkString(option_string) => match option_string {
                Some(string) => write!(f, "BulkString(\"{}\")", string),
                None => write!(f, "BulkString(None)")
            },
            Array(tokens) => {
                write!(f, "Array[").expect("failed to print");
                for (index, token) in tokens.iter().enumerate() {
                    match token.fmt(f) {
                        Ok(_) => {}
                        Err(_) => {
                            panic!("error!")
                        }
                    }
                    let seperator = if index == tokens.len() - 1 { "" } else { ", " };
                    write!(f, "{}", seperator).expect("failed to print");
                }
                write!(f, "]")
            }
        }
    }
}


fn tokenize_resp_string(resp_string: &str) -> std::collections::VecDeque<&str> {
    let mut result = resp_string.split("\r\n").collect::<std::collections::VecDeque<&str>>();
    result.pop_back();
    result
}

fn parse_command_token(command_string: &str) -> (char, &str) {
    if command_string.len() < 2 {
        panic!("Invalid command string.")
    } else {
        (command_string.as_bytes()[0] as char, &command_string[1..])
    }
}

fn parse_one_resp_token<'a>(mut token_strings: std::collections::VecDeque<&'a str>) -> core::result::Result<(RespType, std::collections::VecDeque<&'a str>), &'static str> {
    let first_token = match token_strings.pop_front() {
        Some(token_string) => token_string,
        None => {
            return Err("No token strings in vector slice");
        }
    };
    let (command, command_info) = parse_command_token(first_token);
    match command {
        '+' => {
            Ok((SimpleString(command_info), token_strings))
        }
        '-' => {
            let (error_type, error_message) = command_info.split_once(' ').unwrap();
            Ok((RespError((error_type, error_message)), token_strings))
        }
        ':' => {
            match command_info.parse::<i32>() {
                Ok(value) => Ok((Integer(value), token_strings)),
                Err(_) => Err("failed to parse an integer value")
            }
        }
        '$' => {
            match command_info.parse::<i32>() {
                Ok(expected_string_length) => {
                    if expected_string_length == -1 {
                        Ok((BulkString(None), token_strings))
                    } else if expected_string_length < -1 {
                        return Err("Invalid bulk string length");
                    } else {
                        let string = match token_strings.pop_front() {
                            Some(string) => string,
                            None => {
                                return Err("Bulk string missing.");
                            }
                        };
                        if string.len() != expected_string_length as usize {
                            return Err("Bulk string has incorrect length");
                        }
                        Ok((BulkString(Some(string)), token_strings))
                    }
                }
                Err(_) => Err("Failed to parse string length of bulk string")
            }
        }
        '*' => {
            // let expected_array_length = std::from_str::<i32>(command_info);
            match command_info.parse::<usize>() {
                Ok(expected_array_length) => {
                    let mut acc: std::vec::Vec<Box<RespType>> = std::vec::Vec::new();
                    let mut tail = token_strings;
                    for _ in 0..expected_array_length {
                        match parse_one_resp_token(tail) {
                            Ok((token, _tail)) => {
                                acc.push(Box::new(token));
                                tail = _tail;
                            }
                            Err(error_string) => {
                                return Err(error_string);
                            }
                        }
                    }
                    Ok((Array(acc), tail))
                }
                Err(_) => Err("Failed to parse array length.")
            }
        }
        _ => Err("Invalid token, first character must be *,+,-,: or $")
    }
}

fn _parse_token_strings<'a>(token_strings: std::collections::VecDeque<&'a str>, mut acc: std::vec::Vec<RespType<'a>>) -> std::result::Result<std::vec::Vec<RespType<'a>>, &'static str> {
    if token_strings.len() == 0 {
        Ok(acc)
    } else {
        match parse_one_resp_token(token_strings) {
            Ok((token, tail)) => {
                acc.push(token);
                _parse_token_strings(tail, acc)
            }
            Err(error_string) => Err(error_string)
        }
    }
}

fn parse_token_strings(token_strings: std::collections::VecDeque<&str>) -> std::result::Result<std::vec::Vec<RespType>, &'static str> {
    _parse_token_strings(token_strings, std::vec::Vec::new())
}

fn parse_resp(input_string: &str) -> std::result::Result<std::vec::Vec<RespType>, &'static str> {
    let token_strings = tokenize_resp_string(input_string);
    parse_token_strings(token_strings)
}

