use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};

use crate::internal::ports::services::get_udp_service_name;
use crate::internal::reporter::models::PortResult;
use crate::internal::ports::timeouts::TIMEOUT;

enum PortStatus {
    Open(PortResult),
    Closed,
    Filtered,
}

pub async fn scan_udp_ports(ip: String) -> (Vec<PortResult>, usize, usize) {
    let udp_ports = vec![
        53, 67, 68, 69, 88, 111, 123, 137, 138, 161, 162, 389, 443, 500, 514, 520,
        1194, 1434, 1701, 1812, 1813, 1900, 2049, 3306, 3478, 3702, 4500, 5000, 5060,
        5353, 5432, 5900, 6379, 8080, 8443, 11211, 27017, 51820,
    ];

    let mut open_ports = vec![];
    let mut closed_ports = 0;
    let mut filtered_ports = 0;

    for port in udp_ports {
        match scan_udp_port(&ip, port).await {
            PortStatus::Open(res) => open_ports.push(res),
            PortStatus::Closed => closed_ports += 1,
            PortStatus::Filtered => filtered_ports += 1,
        }
    }

    (open_ports, closed_ports, filtered_ports)
}

async fn scan_udp_port(ip: &str, port: u16) -> PortStatus {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return PortStatus::Closed,
    };

    let target = format!("{}:{}", ip, port);
    let probe = build_probe(port);
    let _ = socket.send_to(&probe, &target).await;

    let mut buffer = [0u8; 1024];
    let result = timeout(
        Duration::from_millis(TIMEOUT),
        socket.recv_from(&mut buffer)
    ).await;

    match result {
        Ok(Ok((_size, _addr))) => {
            PortStatus::Open(PortResult {
                port,
                service: get_udp_service_name(port),
            })
        }
        Ok(Err(_)) => PortStatus::Closed,
        Err(_) => PortStatus::Filtered,
    }
}

fn build_probe(port: u16) -> Vec<u8> {
    match port {
        53 => vec![0x12, 0x34, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        123 => vec![0x1b; 48],
        1900 => b"M-SEARCH * HTTP/1.1\r\n\r\n".to_vec(),
        5353 => vec![0x00, 0x00, 0x00, 0x00],
        161 => vec![0x30, 0x26, 0x02, 0x01, 0x01],
        _ => b"ping".to_vec()
    }
}