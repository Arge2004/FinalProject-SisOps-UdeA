use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};

use crate::internal::ports::services::get_service_name;
use crate::internal::ports::timeouts::{TIMEOUT, CONCURRENCY};
use crate::internal::reporter::models::PortResult;

enum PortStatus {
    Open(PortResult),
    Closed,
    Filtered,
}

pub async fn scan_tcp_ports(ip: String) -> (Vec<PortResult>, usize, usize) {
    let semaphore = Arc::new(Semaphore::new(CONCURRENCY));
    let mut handles = vec![];

    for port in 1..=65535 {
        let ip_clone = ip.clone();
        let semaphore = semaphore.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let address = format!("{}:{}", ip_clone, port);

            let result = timeout(
                Duration::from_millis(TIMEOUT),
                TcpStream::connect(&address)
            ).await;

            match result {
                Ok(Ok(_stream)) => {
                    PortStatus::Open(PortResult {
                        port,
                        service: get_service_name(port),
                    })
                }
                Ok(Err(_)) => PortStatus::Closed,
                Err(_) => PortStatus::Filtered,
            }
        });
        handles.push(handle);
    }

    let mut open_ports = vec![];
    let mut closed_ports = 0;
    let mut filtered_ports = 0;

    for handle in handles {
        if let Ok(status) = handle.await {
            match status {
                PortStatus::Open(pr) => open_ports.push(pr),
                PortStatus::Closed => closed_ports += 1,
                PortStatus::Filtered => filtered_ports += 1,
            }
        }
    }

    (open_ports, closed_ports, filtered_ports)
}