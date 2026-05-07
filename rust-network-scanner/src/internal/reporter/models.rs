#[derive(Debug, Clone)]
pub struct HostResult {
    pub ip: String,
    pub latency: u128,
    pub scan_result: ScanResult,
}


#[derive(Debug, Clone)]
pub struct ScanResult {
    pub tcp_ports:
        Vec<PortResult>,
    pub udp_ports:
        Vec<PortResult>,
}


#[derive(Debug, Clone)]
pub struct PortResult {
    pub port: u16,
    pub service: String,
}