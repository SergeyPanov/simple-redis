use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    num::ParseIntError,
};

#[derive(Debug)]
pub enum RESPLMessage {
    Strint(String),
    Integer(i32),
    Array(Vec<RESPLMessage>),
}

impl RESPLMessage {
    pub fn new(stream: &TcpStream) -> Result<RESPLMessage, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(stream);
        let mut stack: Vec<i32> = vec![];
        let mut respl_message_vec: Vec<RESPLMessage> = vec![];

        stack.push(extract_int(next_line(&mut reader)?)?);

        while !stack.is_empty() {
            match stack.pop() {
                Some(mut size) => {
                    let line = next_line(&mut reader)?;

                    if line.starts_with(":") {
                        // Process int
                        respl_message_vec.push(RESPLMessage::Integer(extract_int(line)?));
                    } else if line.starts_with("$") {
                        // Procees string
                        respl_message_vec.push(RESPLMessage::Strint(next_line(&mut reader)?));
                    } else {
                        // Could process arrays, etc.
                        ()
                    }

                    size -= 1;

                    if size > 0 {
                        stack.push(size);
                    }
                }
                None => break,
            }
        }

        Ok(RESPLMessage::Array(respl_message_vec))
    }
}

fn extract_int(str: String) -> Result<i32, ParseIntError> {
    str.chars()
        .filter(|c| c.is_ascii_digit() || c.eq_ignore_ascii_case(&'-'))
        .collect::<String>()
        .parse::<i32>()
}

fn next_line(reader: &mut BufReader<&TcpStream>) -> Result<String, std::io::Error> {
    let mut buf = String::new();

    match reader.read_line(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}
