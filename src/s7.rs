use crate::DeviceInfo;
pub async fn inspect(target: &str) -> DeviceInfo {
    eprintln!("  Connecting to {} via s7...", target);
    DeviceInfo { ip: target.into(), protocol: "s7".into(), vendor: "Unknown".into(), model: "Unknown".into(), firmware: "Unknown".into(), serial: "N/A".into(), cpu_state: "unknown".into(), modules: vec![], programs: vec![], purdue_level: 2 }
}
