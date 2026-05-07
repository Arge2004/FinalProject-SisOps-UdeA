use std::sync::Arc;

use tokio::net::TcpStream;

use tokio::sync::Semaphore;

use tokio::time::{
    timeout,
    Duration
};

use crate::internal::ports::services::get_service_name;

use crate::internal::ports::timeouts::{
    CONNECT_TIMEOUT_MS,
    MAX_PORT_CONCURRENCY
};

use crate::internal::reporter::models::PortResult;

pub async fn scan_tcp_ports(
    ip: String
) -> Vec<PortResult> {

    let semaphore = Arc::new(
        Semaphore::new(
            MAX_PORT_CONCURRENCY
        )
    );

    let mut handles = vec![];

    for port in 1..=65535 {

        let ip_clone = ip.clone();

        let semaphore =
            semaphore.clone();

        let handle =
            tokio::spawn(async move {

            let _permit =
                semaphore
                .acquire()
                .await
                .unwrap();

            let address =
                format!(
                    "{}:{}",
                    ip_clone,
                    port
                );

            let result = timeout(
                Duration::from_millis(
                    CONNECT_TIMEOUT_MS
                ),

                TcpStream::connect(
                    &address
                )
            ).await;

            match result {

                Ok(Ok(_stream)) => {

                    Some(
                        PortResult {
                            port,
                            service:
                                get_service_name(
                                    port
                                ),
                        }
                    )
                }

                _ => None
            }
        });

        handles.push(handle);
    }

    let mut open_ports = vec![];

    for handle in handles {

        if let Ok(Some(port_result))
            = handle.await {

            open_ports.push(
                port_result
            );
        }
    }

    open_ports
}