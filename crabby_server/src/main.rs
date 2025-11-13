use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to address");

    println!("Crabby server running at http://127.0.0.1:7878");

    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }

    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = match request.lines().next() {
        Some(line) => line,
        None => return,
    };

    
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let path = parts[1];
    
    let (status_line, filename) = if path == "/" {
        ("HTTP/1.1 200 OK\r\n", "static/index.html")
    } else if path == "/page1.html" || path.starts_with("/page1.html?") {
        ("HTTP/1.1 200 OK\r\n", "static/page1.html")
    } else if path == "/page2.html" || path.starts_with("/page2.html?") {
        ("HTTP/1.1 200 OK\r\n", "static/page2.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "static/404.html")
    };




    let contents = if Path::new(filename).exists() {
        fs::read_to_string(filename).unwrap_or_else(|_| String::from("<h1>Error reading file</h1>"))
    } else {
        
        String::from("<h1>404 - Not Found</h1>")
    };

    let response = format!(
        "{status}Content-Length: {length}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{body}",
        status = status_line,
        length = contents.len(),
        body = contents
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    }
}
