// Uncomment this block to pass the first stage
use std::{
    io::Write,
    net::{TcpListener, TcpStream}, thread,
};

use redis_starter_rust::RESPLMessage;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        println!("A new connection");

        // Add a new connection
        match stream {
            Ok(mut _stream) => {
                thread::spawn(|| {
                    handle_request(_stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let pong = "+PONG\r\n";
    
    loop {
        match RESPLMessage::new(&stream) {
            Ok(_message) => {
                println!("{:?}", _message);
                if let Err(e) = stream.write_all(pong.as_bytes()) {
                    println!("Error: {}", e);
                }
            }
            Err(e) => {
                println!("Error occured while message processing: {}", e);
                break;
            }
        }
    }
}
