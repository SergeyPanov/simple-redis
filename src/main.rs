// Uncomment this block to pass the first stage
use std::{
    io::{Write},
    net::{TcpListener},
};

use redis_starter_rust::RESPLMessage;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let response = "+PONG\r\n";

        match stream {
            Ok(mut _stream) => {
                loop {
                    match RESPLMessage::new(&_stream) {
                        Ok(_message) => {
                            println!("{:?}", _message);
                            if let Err(e) = _stream.write_all(response.as_bytes()) {
                                println!("Error: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
