use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_message() -> std::io::Result<()> {
    let mut stream: TcpStream = TcpStream::connect("localhost:3000")?;

    let msg = b"Hello, Server!";
    stream.write(msg)?;

    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let received: Cow<str> = String::from_utf8_lossy(&buffer[..]);
    println!("Received: {}", received);

    Ok(())
}