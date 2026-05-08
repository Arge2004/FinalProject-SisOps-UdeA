use crate::internal::ports::tcp_scanner::scan_tcp_ports;
use crate::internal::ports::udp_scanner::scan_udp_ports;
use crate::internal::reporter::models::ScanResult;

pub async fn scan_all_ports(ip: String) -> (ScanResult, usize, usize, usize) {
    let (tcp_ports, tcp_closed, tcp_filtered) = scan_tcp_ports(ip.clone()).await;
    let (udp_ports, udp_closed, udp_filtered) = scan_udp_ports(ip.clone()).await;

    let scan_result = ScanResult {
        tcp_ports,
        udp_ports,
    };

    let open_ports = scan_result.tcp_ports.len() + scan_result.udp_ports.len();
    let closed_ports = tcp_closed + udp_closed;
    let filtered_ports = tcp_filtered + udp_filtered;

    (scan_result, open_ports, closed_ports, filtered_ports)
}