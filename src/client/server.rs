use std::net::TcpListener;
use std::thread;
// use std::format;

mod handle {
  use std::net::TcpStream;
  use std::io::{Read, Write};
  pub fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
  }
  
  pub fn handle_write(mut stream: TcpStream, response: String) {
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
  }
  
  pub fn handle_client(stream: TcpStream, response: String) {
    handle_read(&stream);
    handle_write(stream, response);
  }
}

pub fn start_server(port: &str, response: String) {
  let listener = TcpListener::bind(port).unwrap();
    println!("Listening for connections on port {}", 8080);
    let ok: String = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n".to_string();
    let to_give = format!("{}{}\n",ok, response);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // thread::spawn(|| {
                    handle::handle_client(stream, to_give.to_owned())
                //});
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}