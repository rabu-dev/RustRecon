## herramienta para analizar de red por ip 

comando 
cargo run -- scan --target 192.168.1.1 --ports 1-4000

resustado
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target\debug\RustRecon.exe scan --target 192.168.1.1 --ports 1-4000`
Host: 192.168.1.1
  - Puerto 80 | Servicio: Some("http") | Versión: None
  - Puerto 21 | Servicio: Some("ftp") | Versión: Some("220---------- Welcome to FTP Server ----------\r\n220-You are user number 2 of 50 allowed.\r\n220-Local time is now 16:32. Server port: 21. IPv6 connections are also welcome on this server.\r\n220 You will be disconnected after 15 minutes of inactivity.")
  - Puerto 53 | Servicio: None | Versión: None
  - Puerto 443 | Servicio: Some("https") | Versión: None
  - Puerto 2601 | Servicio: Some("unknown") | Versión: Some("Vty password isn't set.")
