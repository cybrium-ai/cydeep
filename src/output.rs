use crate::DeviceInfo;
use colored::Colorize;
pub fn print_device(d: &DeviceInfo, format: &str) {
    match format {
        "json" => println!("{}", serde_json::to_string_pretty(d).unwrap()),
        _ => { eprintln!("  {}\n  IP: {} | Protocol: {} | Vendor: {} | Model: {}\n  Firmware: {} | Serial: {} | CPU: {} | Purdue: L{}", "Device".white().bold(), d.ip, d.protocol.cyan(), d.vendor, d.model, d.firmware, d.serial, d.cpu_state, d.purdue_level); }
    }
}
