use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for x in tcp_listener.incoming() {
        let mut stream = x.unwrap();

        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();
        println!("Receive:{:?}", str::from_utf8(&buffer[..len]).unwrap());
        println!("Receive:{}", str::from_utf8(&buffer[..len]).unwrap());

        stream.write(&mut buffer[..len]).unwrap();
    }
}
