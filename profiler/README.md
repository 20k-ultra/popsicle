# Profiler

Profiler is an asynchronous web server profiling library that supports concurrent requests.

Benchmarks the following metrics when talking to a web server:

- DNS resolution (Uses tokio::net::[lookup_host](https://docs.rs/tokio/latest/tokio/net/fn.lookup_host.html))
- TCP connection (Uses tokio::net::[TcpStream](https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html))
- TLS connection (Uses tokio_native_tls::[TlsConnector](https://docs.rs/tokio-native-tls/latest/tokio_native_tls/struct.TlsConnector.html))
- First byte received (Tracks bytes received on tokio_native_tls::[TlsStream](https://docs.rs/tokio-native-tls/latest/tokio_native_tls/struct.TlsStream.html))
- Total response time (Sums all elapsed times)

### Usage Example

```rust
use profiler;

#[tokio::main]
async fn main() {
    let domain = "google.com";
    let concurrency = 3;

    match profiler::benchmark(&domain, concurrency).await {
        Ok(benchmarks) => println!("{:?}", benchmarks),
        Err(e) => panic!("Failed to profile webserver {}", e),
    }
}
```
