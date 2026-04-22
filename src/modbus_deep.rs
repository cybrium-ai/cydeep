use crate::DeviceInfo;
pub async fn inspect(target: &str) -> DeviceInfo {
    eprintln!("  Connecting to {} via modbus_deep...", target);
    DeviceInfo { ip: target.into(), protocol: "modbus_deep".into(), vendor: "Unknown".into(), model: "Unknown".into(), firmware: "Unknown".into(), serial: "N/A".into(), cpu_state: "unknown".into(), modules: vec![], programs: vec![], purdue_level: 2 }
}
