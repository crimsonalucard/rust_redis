extern crate tokio;

use resp_parser::{parse_resp, serialize_resp};
use std::env;
use std::net::SocketAddr;
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Only one argument required to run. Please write port number");
    }
    let port = args[2].parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await
    }
}

async fn process(mut socket: TcpStream) {
    let mut buffer: [u8; 64] = [0; 64];
    socket.read(&mut buffer).await.unwrap();
    let string = from_utf8(&buffer).unwrap();
    let tokens = parse_resp(string).unwrap();
    let echo = serialize_resp(&tokens).unwrap();
    socket.write(echo.as_bytes()).await.unwrap();
}
