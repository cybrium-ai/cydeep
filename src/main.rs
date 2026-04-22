mod s7; mod cip; mod modbus_deep; mod bacnet_deep; mod output;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "cydeep", version, about = "PLC deep inspection — Cybrium AI")]
struct Cli { #[command(subcommand)] command: Commands }
#[derive(Subcommand)]
enum Commands {
    Inspect { #[arg(short, long)] target: String, #[arg(short, long, default_value = "s7")] protocol: String, #[arg(short = 'f', long, default_value = "text")] format: String },
    Discover { #[arg(short, long)] targets: String },
    Version,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo { pub ip: String, pub protocol: String, pub vendor: String, pub model: String, pub firmware: String, pub serial: String, pub cpu_state: String, pub modules: Vec<String>, pub programs: Vec<String>, pub purdue_level: u8 }

fn print_banner() {
    eprintln!("\x1b[35m\n   ___  _   _  ___  ___  ___  ___ \n  / __|| | | ||   \\| __|| __|| _ \\\n | (__ | |_| || |) || _| | _| |  /\n  \\___| \\__, ||___/ |___||___||_|_\\\n        |___/\n\x1b[0m");
    eprintln!("  \x1b[35m\x1b[1mcydeep\x1b[0m v{} — \x1b[2mCybrium AI PLC Inspector\x1b[0m\n", env!("CARGO_PKG_VERSION"));
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Inspect { target, protocol, format } => {
            print_banner();
            let d = match protocol.as_str() { "s7" => s7::inspect(&target).await, "cip" => cip::inspect(&target).await, "modbus" => modbus_deep::inspect(&target).await, _ => bacnet_deep::inspect(&target).await };
            output::print_device(&d, &format);
        }
        Commands::Discover { targets } => { print_banner(); eprintln!("  Scanning {}...", targets); }
        Commands::Version => println!("cydeep {} — Cybrium AI PLC Inspector", env!("CARGO_PKG_VERSION")),
    }
}
