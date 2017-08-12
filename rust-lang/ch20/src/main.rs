extern crate ch20;
use ch20::ThreadPool;

use std::net::TcpListener;
use std::net::TcpStream;

// See https://doc.rust-lang.org/std/io/prelude/
// the prelude imports all the common Read, Write ...
use std::io::prelude::*;
//use std::io::Read;

use std::fs::File;
use std::thread;

fn main() {
    let address = "127.0.0.1:8080";

    let listener = TcpListener::bind(address).unwrap();
	let pool = ThreadPool::new(4);
	
    for streamResult in listener.incoming() {
        // Result<TcpStream>
        match streamResult {
            Ok(stream) => {
                // stream: TcpStream
//                println!("New client");
				pool.execute(||{
				    handle_connection(stream);
				});
                
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
            println!("---------------------------------");
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            println!("---------------------------------");
			
            let get = b"GET / HTTP/1.1\r\n";
			
			let (status, filename) = if buffer.starts_with(get) {
				("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
			} else {
				("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
			};

			let mut file = File::open(filename).unwrap();
            let mut content = String::new();
            let length = file.read_to_string(&mut content).unwrap();
            println!("HTML length: {}", length);
            let response = format!("{}{}", status, content);
            let length = stream.write(response.as_bytes()).unwrap();
            println!("bytes sent: {}", length);
            stream.flush().unwrap();

//            if buffer.starts_with(get) {
//                let mut file = File::open("hello.html").unwrap();
//                let mut content = String::new();
//                let length = file.read_to_string(&mut content).unwrap();
//                println!("HTML length: {}", length);
//
//                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
//                let length = stream.write(response.as_bytes()).unwrap();
//                println!("bytes sent: {}", length);
//                stream.flush().unwrap();
//            } else {
//				let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
//				let mut file = File::open("404.html").unwrap();
//				let mut content = String::new();
//				file.read_to_string(&mut content).unwrap();
//				
//				let response = format!("{}{}", header, content);
//				stream.write(response.as_bytes()).unwrap();
//				stream.flush().unwrap();
//            }

        }
        Err(err) => {}
    }
}
