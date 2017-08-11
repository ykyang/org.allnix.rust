use std::net::TcpListener;
use std::net::TcpStream;

// See https://doc.rust-lang.org/std/io/prelude/
//use std::io::prelude::*;

use std::io::Read;

fn main() {
    let address = "127.0.0.1:8080";

    let listener = TcpListener::bind(address).unwrap();

    for streamResult in listener.incoming() { // Result<TcpStream>
        match streamResult {
            Ok(stream) => { // stream: TcpStream
                println!("New client");
                handle_connection(stream);
            }
            Err(err) => {
                println!("Connection failed");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // array size of 512 and filled with 0's
    // fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    match stream.read(&mut buffer) {
        Ok(length) => {
            let response = 
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }
        Err(err) => {

        }
    }
}
