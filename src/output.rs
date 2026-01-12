use crate::models::Host;

pub fn print_console(host: &Host) {
    if host.ports.is_empty() {
        println!(
            "Host {} accesible, sin puertos abiertos detectados",
            host.ip
        );
        return;
    }

    println!("Host: {}", host.ip);
    for port in &host.ports {
        println!(
            "  - Puerto {} | Servicio: {:?} | Versión: {:?}",
            port.port,
            port.service,
            port.version
        );
    }
}

pub fn print_json(host: &Host) {
    let json = serde_json::to_string_pretty(host).unwrap();
    println!("{}", json);
}
