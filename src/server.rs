use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write, Result};

pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 { return Ok(()); } // connection closed

        stream.write(&buffer[..bytes_read])?;
        stream.flush()?;
    }
}

pub fn start_server() -> Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;

    println!("Server is listening on localhost:3000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}