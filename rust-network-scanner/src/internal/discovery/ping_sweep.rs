use surge_ping::{Client, Config, PingIdentifier, PingSequence};

use std::net::IpAddr;
use std::time::Duration;

use crate::internal::ports::scanner::scan_all_ports;
use crate::internal::reporter::models::HostResult;

pub async fn discover_host(ip: String) -> Option<HostResult> {

    let addr: IpAddr = match ip.parse() {
        Ok(ip) => ip,
        Err(_) => return None,
    };

    let config = Config::default();

    let client = match Client::new(&config) {
        Ok(client) => client,
        Err(_) => return None,
    };

    let mut pinger = client
        .pinger(addr, PingIdentifier(111))
        .await;

    pinger.timeout(Duration::from_millis(500));

    match pinger.ping(PingSequence(0), &[]).await {

        Ok((_packet, duration)) => {
            let scan_result =
                scan_all_ports(
                    ip.clone()
                ).await;
            Some(
                HostResult {
                    ip,
                    latency: duration
                        .as_millis(),
                    scan_result,
                }
            )
        }

        Err(_) => None
    }
}