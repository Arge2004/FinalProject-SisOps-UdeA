use tokio::net::UdpSocket;

use tokio::time::{
    timeout,
    Duration
};

use crate::internal::ports::services::get_udp_service_name;

use crate::internal::reporter::models::PortResult;

pub async fn scan_udp_ports(
    ip: String
) -> Vec<PortResult> {

    // Lista de puertos UDP conocidos
    let udp_ports = vec![

        53,
        67,
        68,
        69,
        88,
        111,
        123,
        137,
        138,
        161,
        162,
        389,
        443,
        500,
        514,
        520,
        1194,
        1434,
        1701,
        1812,
        1813,
        1900,
        2049,
        3306,
        3478,
        3702,
        4500,
        5000,
        5060,
        5353,
        5432,
        5900,
        6379,
        8080,
        8443,
        11211,
        27017,
        51820,
    ];

    let mut open_ports = vec![];

    for port in udp_ports {

        if let Some(result) =
            scan_udp_port(
                &ip,
                port
            ).await {

            open_ports.push(result);
        }
    }

    open_ports
}

async fn scan_udp_port(
    ip: &str,
    port: u16
) -> Option<PortResult> {

    let socket =
        UdpSocket::bind("0.0.0.0:0")
        .await
        .ok()?;

    let target =
        format!("{}:{}", ip, port);

    // Probe simple
    let probe =
        build_probe(port);

    let _ = socket
        .send_to(
            &probe,
            &target
        )
        .await;

    let mut buffer = [0u8; 1024];

    let result = timeout(
        Duration::from_millis(1000),

        socket.recv_from(&mut buffer)
    ).await;

    match result {

        Ok(Ok((_size, _addr))) => {
            Some(
                PortResult {
                    port,
                    service:
                        get_udp_service_name(
                            port
                        ),
                }
            )
        }

        _ => None
    }
}

fn build_probe(
    port: u16
) -> Vec<u8> {

    match port {

        // DNS
        53 => vec![
            0x12, 0x34,
            0x01, 0x00,
            0x00, 0x01,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00
        ],

        // NTP
        123 => vec![0x1b; 48],

        // SSDP
        1900 => b"M-SEARCH * HTTP/1.1\r\n\r\n"
            .to_vec(),

        // mDNS
        5353 => vec![
            0x00, 0x00,
            0x00, 0x00
        ],

        // SNMP
        161 => vec![
            0x30, 0x26,
            0x02, 0x01,
            0x01
        ],

        _ => b"ping".to_vec()
    }
}