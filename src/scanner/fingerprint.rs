use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn fingerprint(ip: &str, port: u16) -> (Option<String>, Option<String>) {
    match port {
        21 => ftp(ip, port).await.unwrap_or((None, None)),
        80 | 8080 => http(ip, port).await.unwrap_or((None, None)),
        22 => ssh(ip, port).await.unwrap_or((None, None)),
        443 => (Some("https".into()), None),
        _ => banner(ip, port).await.unwrap_or((None, None)),
    }
}

async fn http(ip: &str, port: u16) -> Option<(Option<String>, Option<String>)> {
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(&addr).await.ok()?;

    stream
        .write_all(b"HEAD / HTTP/1.0\r\n\r\n")
        .await
        .ok()?;

    let mut buffer = [0u8; 1024];
    let n = timeout(Duration::from_millis(800), stream.read(&mut buffer))
        .await
        .ok()?
        .ok()?;

    let response = String::from_utf8_lossy(&buffer[..n]);

    let server = response
        .lines()
        .find(|l| l.to_lowercase().starts_with("server:"))
        .map(|l| l.replace("Server:", "").trim().to_string());

    Some((Some("http".into()), server))
}

async fn ssh(ip: &str, port: u16) -> Option<(Option<String>, Option<String>)> {
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(&addr).await.ok()?;

    let mut buffer = [0u8; 256];
    let n = timeout(Duration::from_millis(800), stream.read(&mut buffer))
        .await
        .ok()?
        .ok()?;

    let banner = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

    Some((Some("ssh".into()), Some(banner)))
}

async fn ftp(ip: &str, port: u16) -> Option<(Option<String>, Option<String>)> {
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(&addr).await.ok()?;

    let mut buffer = [0u8; 1024];
    let n = timeout(Duration::from_millis(800), stream.read(&mut buffer))
        .await
        .ok()?
        .ok()?;

    let response = String::from_utf8_lossy(&buffer[..n]);

    if response.starts_with("220") {
        Some((Some("ftp".into()), Some(response.trim().to_string())))
    } else {
        None
    }
}

async fn banner(ip: &str, port: u16) -> Option<(Option<String>, Option<String>)> {
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(&addr).await.ok()?;

    let mut buffer = [0u8; 256];
    let n = timeout(Duration::from_millis(800), stream.read(&mut buffer))
        .await
        .ok()?
        .ok()?;

    let banner = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

    Some((Some("unknown".into()), Some(banner)))
}
