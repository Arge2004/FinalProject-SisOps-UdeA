use crate::internal::ports::tcp_scanner::scan_tcp_ports;

use crate::internal::ports::udp_scanner::scan_udp_ports;

use crate::internal::reporter::models::{
    ScanResult
};

pub async fn scan_all_ports(
    ip: String
) -> ScanResult {

    let tcp_ports =
        scan_tcp_ports(
            ip.clone()
        ).await;

    let udp_ports =
        scan_udp_ports(
            ip.clone()
        ).await;

    ScanResult {

        tcp_ports,

        udp_ports,
    }
}