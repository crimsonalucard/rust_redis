extern crate tokio;

use database::{create_new_db, Db};
use redis_commands::handle_resp_token;
use resp_parser::parse_resp;
use std::env;
use std::net::SocketAddr;
use std::str::from_utf8;
use std::sync::Arc;
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
    let db = Arc::new(create_new_db());
    loop {
        let db_ref = db.clone();
        let (socket, _) = listener.accept().await.unwrap();
        let _handle = tokio::spawn(async move { process(socket, db_ref).await });
    }
}

async fn process<'a>(mut socket: TcpStream, db: Arc<Db>) {
    let mut buffer: [u8; 64] = [0; 64];
    let size = socket.read(&mut buffer).await.unwrap();
    let string = from_utf8(&buffer[..size]).unwrap();
    let mut tokens = parse_resp(string).unwrap();
    // let echo = serialize_resp(&tokens).unwrap();
    // println!("{}", echo);
    let resp_response = handle_resp_token(&mut tokens[0], db);
    socket.write(resp_response.as_bytes()).await.unwrap();
    socket.shutdown().await.unwrap();
}
