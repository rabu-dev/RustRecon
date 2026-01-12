use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "iron-scan", version, about = "Enterprise Network Scanner")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Scan {
        #[arg(short, long)]
        target: String,

        #[arg(short, long, default_value = "1-1024")]
        ports: String,

        #[arg(long, default_value = "500")]
        timeout: u64,

        #[arg(long)]
        json: bool,
    },
}

pub fn parse_ports(input: &str) -> Result<Vec<u16>, String> {
    let mut ports = Vec::new();

    for part in input.split(',') {
        let part = part.trim();

        if part.contains('-') {
            let mut range = part.split('-');

            let start: u16 = range
                .next()
                .ok_or("Rango inválido")?
                .parse()
                .map_err(|_| format!("Puerto inválido: {}", part))?;

            let end: u16 = range
                .next()
                .ok_or("Rango inválido")?
                .parse()
                .map_err(|_| format!("Puerto inválido: {}", part))?;

            if start > end {
                return Err(format!("Rango inválido: {}", part));
            }

            for p in start..=end {
                ports.push(p);
            }
        } else {
            let port: u16 = part
                .parse()
                .map_err(|_| format!("Puerto inválido: {}", part))?;
            ports.push(port);
        }
    }

    ports.sort_unstable();
    ports.dedup();

    Ok(ports)
}
