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
    let port = args[1].parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    let sockets: Vec<TcpStream> = vec![];
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let handle = tokio::spawn(async move { process(socket).await });
    }
}

async fn process(mut socket: TcpStream) {
    let mut buffer: [u8; 64] = [0; 64];
    let size = socket.read(&mut buffer).await.unwrap();
    let string = from_utf8(&buffer[..size]).unwrap();
    let tokens = parse_resp(string).unwrap();
    let echo = serialize_resp(&tokens).unwrap();
    println!("{}", echo);
    socket.write(echo.as_bytes()).await.unwrap();
    socket.shutdown().await.unwrap();
}
