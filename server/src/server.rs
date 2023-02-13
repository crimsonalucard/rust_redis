use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};

pub fn launch_server(port: u16) {
    let address = SocketAddr::from(([127,0,0,1], port));
    let listener = TcpListener::bind(&address).unwrap();
    let mut buffer: [u8; 100] = [0; 100];
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let size = stream.read(&mut buffer).unwrap();
                {
                    copy&mut buffer[size..]
                }
                stream.write(&buffer[0..size]).unwrap();
            }, _ => ()
        }
    }
}