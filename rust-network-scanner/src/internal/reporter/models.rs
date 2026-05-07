#[derive(Debug, Clone)]
pub struct PortResult {

    pub port: u16,

    pub service: String,
}

#[derive(Debug, Clone)]
pub struct HostResult {

    pub ip: String,

    pub latency: u128,

    pub open_ports: Vec<PortResult>,
}