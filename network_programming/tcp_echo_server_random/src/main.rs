// Network Programming with Rust
// Chapter 3 - tcp_echo_server

extern crate rand;

use std::net::{TcpListener, TcpStream};
use std::thread;
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::io::{Read, Write, Error};

// Handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()); }
        let sleep = Duration::from_secs(*thread_rng()
                                    .choose(&[0,1, 2, 3, 4, 5])
                                    .unwrap());
        println!("Sleeping for {:?} before replying", sleep);
        std::thread::sleep(sleep);
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888")
                        .expect("could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
