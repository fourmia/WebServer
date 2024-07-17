use std::io::{Read,Write};
use std::net::TcpListener;
use log::{info, error};
use env_logger;

fn main() {
    env_logger::init();
    std::env::set_var("RUST_LOG", "info");
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    info!("Running on port 3000... ");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        info!("Connection established!");
        let mut buffer = [0; 1024];
        let length = stream.read(&mut buffer).unwrap();
        info!("buffer len is:{}, length is:{}",buffer.len(),length);
        let push_str = String::from(" rust.");
        let push_byte = push_str.as_bytes();
        info!("push_byte is:{}", &push_byte.len());
        for i in length..=length+push_byte.len() {
            buffer[i] = push_byte[i-length];
        }
        stream.write(&buffer[..length+push_byte.len()]).unwrap();

    }
    
}