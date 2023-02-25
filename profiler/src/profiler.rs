use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Benchmark {
    pub tcp_handshake: Duration,
    pub tls_handshake: Duration,
    pub first_byte: Duration,
    pub total: Duration,
}

impl Benchmark {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn profile(domain: &str) -> Result<Benchmark, std::io::Error> {
    let profile_start = Instant::now();
    // Create TCP connection
    let address = format!("{}:443", domain);
    let tcp_socket = TcpStream::connect(address).unwrap();
    let tcp_connection_time = profile_start.elapsed();
    // Create TLS connection
    let connector = TlsConnector::new().unwrap();
    let mut tls_stream = connector.connect(domain, tcp_socket).unwrap();
    let tls_connection_time = profile_start.elapsed() - tcp_connection_time;
    // Request data
    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept-Encoding: identity\r\n\r\n",
        "/", domain
    );
    // Write the HTTP request to the stream
    tls_stream.write_all(req.as_bytes()).unwrap();
    // Read the HTTP response from the stream
    let mut response = Vec::new();
    let mut bytes_read = 0;
    let mut first_byte_time = Duration::new(0, 0);
    loop {
        match tls_stream.read_to_end(&mut response) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                if bytes_read == 0 {
                    first_byte_time =
                        profile_start.elapsed() - tcp_connection_time - tls_connection_time;
                }
                bytes_read += n;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    // Return collected durations
    Ok(Benchmark {
        tcp_handshake: tcp_connection_time,
        tls_handshake: tls_connection_time,
        first_byte: first_byte_time,
        total: tcp_connection_time + tls_connection_time + first_byte_time,
    })
}
