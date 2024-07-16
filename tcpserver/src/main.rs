use std::io::{Read,Write};
use std::net::TcpListener;
use log::{info, error};
use env_logger;

fn main() {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    info!("Running on port 3000... ");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        info!("Connection established!");
        let mut buffer = [0;1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
    
}