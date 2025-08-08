use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Result, Write};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(&mut stream).unwrap();
    }
}

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let contents: String;

    if buffer.starts_with(get) {  
        contents = fs::read_to_string("static/html/index.html")?;
        
    } else {
        contents = fs::read_to_string("static/html/404.html")?;
    }

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Lenght: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}
