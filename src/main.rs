mod scanner;
mod cli;
mod models;
mod output;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            target,
            ports,
            timeout,
            json,
        } => {
            // ✅ PARSEO SEGURO AQUÍ
            let ports = match cli::parse_ports(&ports) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error en puertos: {}", e);
                    std::process::exit(1);
                }
            };

            // ✅ LLAMADA LIMPIA AL SCANNER
            let result = scanner::run_scan(
                target,
                ports,
                timeout,
            ).await;

            if json {
                output::print_json(&result);
            } else {
                output::print_console(&result);
            }
        }
    }
}
