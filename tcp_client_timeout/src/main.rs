// Network Programming with Rust
// Chapter 3 - tcp_client

use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();

//    let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8888); 
//    assert_eq!("127.0.0.1:8888".parse(), Ok(remote));

    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1))
                .expect("Could not connect to server");
    stream.set_read_timeout(Some(Duration::from_secs(5)))
                .expect("Could not set a read timeout");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input)
                    .expect("Failed to read from stdin");
        stream.write(input.as_bytes())
                    .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
                    .expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer)
                    .expect("Could not write buffer aas string"));
    }
}
