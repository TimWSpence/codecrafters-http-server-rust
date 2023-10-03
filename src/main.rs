// Uncomment this block to pass the first stage
use anyhow::{anyhow, Result};
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut tx) => {
                println!("accepted new connection");
                let mut rx = BufReader::new(tx.try_clone().unwrap());
                let req_line = parse_request_line(&mut rx).unwrap();
                match req_line.path.as_ref() {
                    "/" => tx.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
                    _ => tx.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap(),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn parse_request_line(s: &mut BufReader<TcpStream>) -> Result<RequestLine> {
    let mut buf = String::new();
    s.read_line(&mut buf)?;
    let (init, r) = &buf.split_at(buf.len() - 2);
    assert_eq!(r, &"\r\n");
    match init.to_lowercase() {
        x if x.starts_with("get ") => Ok(RequestLine {
            method: Method::Get,
            path: x[4..].to_string(),
        }),
        _ => Err(anyhow!("invalid request")),
    }
}

struct RequestLine {
    method: Method,
    path: String,
}

enum Method {
    Get,
}
