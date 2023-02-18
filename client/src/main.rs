use redis_commands::cli_tokens_to_resp;
use resp_parser::serialize_resp;
use std::env;
use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpListener};
// use tokio::net::TcpListener;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Only one argument required to run. Please write port number");
    }
    let port = args[1].parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();
    let (mut stream, _) = listener.accept().unwrap();
    let prompt = "> ";
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let cli_tokens = line.split(" ").collect::<Vec<&str>>();
        let resp_token = cli_tokens_to_resp(cli_tokens);
        let tokens = vec![resp_token];
        let serialized_resp = serialize_resp(&tokens).unwrap();
        let serialized_resp_bytes = serialized_resp.as_bytes();
        stream.write(serialized_resp_bytes).unwrap();
        // dbg!(cli_tokens);
    }
}
