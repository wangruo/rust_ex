use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    stream.write("hello".as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();

    println!("Response from server: {:?}", str::from_utf8(&buffer[..len]).unwrap());
    println!("Response from server: {}", str::from_utf8(&buffer[..len]).unwrap());
}
