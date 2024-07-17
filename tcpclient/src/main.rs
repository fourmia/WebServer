use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use log::{info,error};
use env_logger;

fn main() {
    env_logger::init();
    std::env::set_var("RUST_LOG", "info");
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 11];
    stream.read(&mut buffer).unwrap();
    info!("Reasponse from server:{:#?}", str::from_utf8(&buffer).unwrap());
}