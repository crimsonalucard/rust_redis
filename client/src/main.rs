use redis_commands::cli_tokens_to_resp;
use resp_parser::{parse_resp, serialize_resp};
use std::env;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
// use tokio::net::TcpListener;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Only one argument required to run. Please write port number");
    }
    let port = args[1].parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let prompt = "> ";
    loop {
        let mut stream = TcpStream::connect(addr).unwrap();
        let mut read_buffer: [u8; 64] = [0; 64];
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let trimmed_line = line.trim();
        let cli_tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
        let resp_token = cli_tokens_to_resp(cli_tokens);
        let tokens = vec![resp_token];
        let serialized_resp = serialize_resp(&tokens).unwrap();
        let serialized_resp_bytes = serialized_resp.as_bytes();

        loop {
            match (
                stream.write(serialized_resp_bytes),
                stream.read(&mut read_buffer[0..]),
            ) {
                (Ok(_written_size), Ok(recieved_size)) => {
                    let resp_response_string =
                        std::str::from_utf8(&read_buffer[..recieved_size]).unwrap();
                    let resp_response_tokens = parse_resp(resp_response_string).unwrap();
                    println!("{}", resp_response_tokens[0]);
                    break;
                }
                (Ok(_), Err(e)) => {
                    println!("error on receive: {}", e.to_string());
                    stream = TcpStream::connect(addr).unwrap();
                }
                (Err(e), Ok(_)) => {
                    println!("error on write: {}", e.to_string());
                    stream = TcpStream::connect(addr).unwrap();
                }
                (Err(e1), Err(e2)) => {
                    println!("error on receive: {}", e1.to_string());
                    println!("error on receive: {}", e2.to_string());
                    stream = TcpStream::connect(addr).unwrap();
                }
            }
        }
    }
}
