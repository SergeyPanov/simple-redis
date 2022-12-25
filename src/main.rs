// Uncomment this block to pass the first stage
use std::{
    io::{Write, BufReader, BufRead},
    net::{TcpListener, TcpStream},
};

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
                    read_resp_request(&_stream);
                    if let Err(e) = _stream.write_all(response.as_bytes()) {
                        println!("err: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn read_resp_request(stream: &TcpStream) {
    let mut reader = BufReader::new(stream);
    let len = resp_array_len(&mut reader);

    for _ in 0..len {
        let mut buf = String::new();

        match reader.read_line(&mut buf) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {}", e)
            },
        }
    }
}

fn resp_array_len(reader: &mut BufReader<&TcpStream>) -> i32 {
    let mut buf = String::new();

    match reader.read_line(&mut buf) {
        Ok(_) => buf
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or_default(),
        Err(_) => 0,
    }
}