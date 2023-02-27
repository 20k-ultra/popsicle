use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{lookup_host, TcpStream};

#[derive(Debug, Serialize, Deserialize)]
pub struct Benchmark {
    pub dns_resolution: Duration,
    pub tcp_handshake: Duration,
    pub tls_handshake: Duration,
    pub first_byte: Duration,
    pub total: Duration,
}

pub async fn benchmarks(
    domain: &str,
    concurrency: usize,
) -> Result<Vec<Benchmark>, std::io::Error> {
    let handle = tokio::runtime::Handle::current();
    let mut tasks = Vec::new();
    let domain = Arc::new(domain.to_string());
    for _ in 0..concurrency {
        let domain_copy = domain.clone();
        tasks.push(handle.spawn(async move { benchmark(domain_copy).await }));
    }
    let mut results = Vec::new();
    for handle in tasks {
        results.push(handle.await?.unwrap());
    }
    Ok(results)
}

async fn benchmark(domain: Arc<String>) -> Result<Benchmark, io::Error> {
    let profile_start = Instant::now();
    // Resolve DNS
    let ip_addr = lookup_host(format!("{}:443", &*domain))
        .await?
        .next()
        .unwrap();
    let dns_resolution_time = profile_start.elapsed();
    // Create TCP connection
    let tcp_socket = TcpStream::connect(ip_addr).await?;
    let tcp_connection_time = profile_start.elapsed() - dns_resolution_time;
    // Create TLS connection
    let native_tls_connector = native_tls::TlsConnector::new().unwrap();
    let tls_connector = tokio_native_tls::TlsConnector::from(native_tls_connector);
    let mut tls_stream = tls_connector.connect(&*domain, tcp_socket).await.unwrap();
    let tls_connection_time = profile_start.elapsed() - dns_resolution_time - tcp_connection_time;
    // Request data
    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept-Encoding: identity\r\n\r\n",
        "/", domain
    );
    // Write the HTTP request to the stream
    tls_stream.write_all(req.as_bytes()).await?;
    // Read the HTTP response from the stream
    let mut response = Vec::new();
    let mut bytes_read = 0;
    let mut first_byte_time = Duration::new(0, 0);
    loop {
        match tls_stream.read_buf(&mut response).await? {
            0 => {
                break;
            }
            n => {
                if bytes_read == 0 {
                    // Capture time for first byte received
                    first_byte_time = profile_start.elapsed()
                        - dns_resolution_time
                        - tcp_connection_time
                        - tls_connection_time;
                }
                bytes_read += n;
            }
        }
    }
    // Return collected durations
    Ok(Benchmark {
        dns_resolution: dns_resolution_time,
        tcp_handshake: tcp_connection_time,
        tls_handshake: tls_connection_time,
        first_byte: first_byte_time,
        total: dns_resolution_time + tcp_connection_time + tls_connection_time + first_byte_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn triple_benchmark() {
        match benchmarks("google.com", 3).await {
            Ok(b) => assert_eq!(b.len(), 3),
            Err(e) => panic!("Something went wrong: {}", e),
        }
    }
}
