use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn create_server<'a>(
    port: u16,
    handler: fn(&mut TcpStream, &'a mut [u8]) -> (),
) -> Box<dyn FnOnce() -> ()> {
    Box::new(move || {
        let address = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(&address).unwrap();
        let mut buffer: [u8; 100] = [0; 100];
        for result_stream in listener.incoming() {
            match result_stream {
                Ok(mut stream) => handler(&mut stream, &mut buffer),
                Err(e) => {
                    panic!("failed to create server: {}", e);
                }
            }
        }
    })
}

pub fn create_echo_server<'a>(port: u16) -> Box<dyn FnOnce() -> ()> {
    create_server(port, echo_handler)
}

fn echo_handler(stream: &mut TcpStream, mut buffer: &mut [u8]) {
    let size = stream.read(&mut buffer).unwrap();
    stream.write(&buffer[0..size]).unwrap();
}
