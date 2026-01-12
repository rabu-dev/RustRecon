use crate::models::{Host, Port};
use futures::stream::{self, StreamExt};
mod fingerprint;

use crate::scanner::fingerprint::fingerprint;

pub async fn run_scan(
    ip: String,
    ports: Vec<u16>,
    timeout_ms: u64,
) -> Host {
    let concurrency_limit = 200;

    let open_ports: Vec<Port> = stream::iter(ports)
        .map(|port| {
            let ip = ip.clone();
            async move {
                scan_port(&ip, port, timeout_ms).await
            }
        })
        .buffer_unordered(concurrency_limit)
        .filter_map(|p| async move { p })
        .collect()
        .await;

    Host {
        ip,
        ports: open_ports,
    }
}


async fn scan_port(
    ip: &str,
    port: u16,
    timeout_ms: u64,
) -> Option<Port> {
    use tokio::net::TcpStream;
    use tokio::time::{timeout, Duration};

    let addr = format!("{}:{}", ip, port);

    if timeout(
        Duration::from_millis(timeout_ms),
        TcpStream::connect(addr),
    )
    .await
    .is_ok()
    {
        let (service, version) = fingerprint(ip, port).await;
        Some(Port {
            port,
            service,
            version,
        })
    } else {
        None
    }
}