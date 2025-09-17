//! Syros - Distributed Coordination Service
//!
//! This is the main entry point for the Syros, a distributed coordination
//! service that provides distributed locks, saga orchestration, event sourcing,
//! and caching capabilities for microservices architectures.

use syros::{cli, server};

/// Main entry point for the Syros application.
///
/// This function parses command line arguments and executes the appropriate
/// command based on the user's input. It supports starting the server with
/// various configurations, validating configuration files, and displaying
/// system information.
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error if something goes wrong.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse_args();

    match cli.command {
        Some(cli::Commands::Start {
            port,
            host,
            grpc_port,
            websocket_port,
            servers,
            interface,
        }) => {
            if !cli.quiet {
                println!("Syros - Distributed Coordination Service");
                println!("Version: {}", env!("CARGO_PKG_VERSION"));

                if cli.verbose {
                    println!(
                        "Environment: {}",
                        if cfg!(debug_assertions) {
                            "Development"
                        } else {
                            "Production"
                        }
                    );
                    println!("Mode: Verbose");
                    println!("Config: {}", cli.config);
                    println!("Host: {}", host);
                    println!(
                        "Ports: REST={}, gRPC={}, WebSocket={}",
                        port, grpc_port, websocket_port
                    );
                    println!("Servers: {:?}", servers);
                    if let Some(iface) = &interface {
                        println!("Interface: {}", iface);
                    }
                }
            }

            server::start_server(
                cli.verbose,
                cli.quiet,
                host,
                port,
                grpc_port,
                websocket_port,
                servers,
                interface,
            )
            .await?;
        }
        Some(cli::Commands::Config { validate }) => {
            println!("Checking configuration...");
            if validate {
                match syros::config::Config::load() {
                    Ok(config) => {
                        println!("Configuration valid!");
                        if cli.verbose {
                            println!("Details:");
                            println!("   - Server: {}:{}", config.server.host, config.server.port);
                            println!(
                                "   - gRPC: {}:{}",
                                config.server.host, config.server.grpc_port
                            );
                            println!(
                                "   - WebSocket: {}:{}",
                                config.server.host, config.server.websocket_port
                            );
                        }
                    }
                    Err(e) => {
                        println!("Configuration error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Some(cli::Commands::Info) => {
            println!("Syros - Distributed Coordination Service");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!(
                "Environment: {}",
                if cfg!(debug_assertions) {
                    "Development"
                } else {
                    "Production"
                }
            );
            println!(
                "Rust: {}",
                std::env::var("RUSTC_SEMVER").unwrap_or_else(|_| "Unknown".to_string())
            );
            println!(
                "Target: {}",
                std::env::var("TARGET").unwrap_or_else(|_| "Unknown".to_string())
            );
        }
        None => {
            if !cli.quiet {
                println!("Syros - Distributed Coordination Service");
                println!("Version: {}", env!("CARGO_PKG_VERSION"));

                if cli.verbose {
                    println!(
                        "Environment: {}",
                        if cfg!(debug_assertions) {
                            "Development"
                        } else {
                            "Production"
                        }
                    );
                    println!("Mode: Verbose");
                    println!("Config: {}", cli.config);
                }
            }

            server::start_server(
                cli.verbose,
                cli.quiet,
                "0.0.0.0".to_string(),
                8080,
                9090,
                8081,
                vec![cli::ServerType::All],
                None,
            )
            .await?;
        }
    }

    Ok(())
}
