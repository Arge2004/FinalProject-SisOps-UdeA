use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostResult {
    pub ip: String,
    pub latency: u128,
    pub open_ports: usize,
    pub closed_ports: usize,
    pub filtered_ports: usize,
    pub scan_result: ScanResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub tcp_ports: Vec<PortResult>,
    pub udp_ports: Vec<PortResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortResult {
    pub port: u16,
    pub service: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkReport {
    pub interface: String,
    pub ip: String,
    pub netmask: String,
    pub cidr: u8,
    pub workers: usize,
    pub duration_ms: u128,
    pub hosts: Vec<HostResult>,
}