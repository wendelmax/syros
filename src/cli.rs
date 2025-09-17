//! Command Line Interface module for the Syros.
//!
//! This module defines the CLI structure and argument parsing using the `clap` crate.
//! It provides commands for starting the server, validating configuration,
//! and displaying system information.

use clap::{Parser, Subcommand};

/// Main CLI structure for the Syros.
///
/// This struct defines the global command line arguments that are available
/// across all subcommands, including verbose mode, quiet mode, and configuration file path.
#[derive(Parser)]
#[command(name = "syros")]
#[command(about = "Syros - Distributed Coordination Service")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Enable verbose mode - shows detailed information
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable quiet mode - shows only essential messages
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Configuration file path
    #[arg(short, long, default_value = "config/default.toml", global = true)]
    pub config: String,

    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands for the Syros CLI.
///
/// This enum defines all the subcommands that can be executed by the CLI,
/// including starting the server, validating configuration, and displaying system information.
#[derive(Subcommand)]
pub enum Commands {
    /// Start the server with specified configuration
    Start {
        /// REST server port
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Server host/IP (0.0.0.0 for all IPs, localhost for local only)
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        /// gRPC server port
        #[arg(long, default_value = "9090")]
        grpc_port: u16,

        /// WebSocket server port
        #[arg(long, default_value = "8081")]
        websocket_port: u16,

        /// Servers to start (rest, grpc, websocket, all)
        #[arg(long, default_value = "all", value_delimiter = ',')]
        servers: Vec<ServerType>,

        /// Specific network interface (e.g., eth0, wlan0, 192.168.1.100)
        #[arg(long)]
        interface: Option<String>,
    },
    /// Validate the configuration file
    Config {
        /// Validate the configuration file
        #[arg(short, long)]
        validate: bool,
    },
    /// Display system information
    Info,
}

/// Types of servers that can be started by the Syros.
///
/// This enum defines the different server types that can be selected
/// when starting the platform, allowing for flexible deployment configurations.
#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum ServerType {
    /// REST API server
    Rest,
    /// gRPC server
    Grpc,
    /// WebSocket server
    Websocket,
    /// All servers
    All,
}

/// Parse command line arguments and return the CLI structure.
///
/// This function uses the `clap` crate to parse command line arguments
/// and return a structured representation of the parsed arguments.
///
/// # Returns
///
/// Returns a `Cli` struct containing the parsed command line arguments.
pub fn parse_args() -> Cli {
    Cli::parse()
}
