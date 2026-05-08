use std::sync::Arc;
use tokio::sync::Semaphore;
use std::time::Instant;

use rust_network_scanner::internal::network::ipcalc::{
    get_local_ip,
    generate_ips
};

use rust_network_scanner::internal::discovery::ping_sweep::{
    discover_host
};

use rust_network_scanner::internal::reporter::models::{
    HostResult, NetworkReport
};

use rust_network_scanner::internal::ports::timeouts::CONCURRENCY;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let local_ip = get_local_ip();
    let ips = generate_ips(&local_ip);

    // The current generate_ips logic only works for /24 subnets.
    let interface_name = String::from("eth0");
    let netmask_str = String::from("255.255.255.0");
    let cidr_num = 24;

    let workers_count = CONCURRENCY;
    let semaphore = Arc::new(Semaphore::new(workers_count));

    let mut handles = vec![];

    for ip in ips {
        let semaphore = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            discover_host(ip).await
        });
        handles.push(handle);
    }

    let mut active_hosts: Vec<HostResult> = vec![];

    for handle in handles {
        if let Ok(Some(host)) = handle.await {
            active_hosts.push(host);
        }
    }

    active_hosts.sort_by(|a, b| a.ip.cmp(&b.ip));

    let duration_ms = start.elapsed().as_millis();

    let report = NetworkReport {
        interface: interface_name,
        ip: local_ip,
        netmask: netmask_str,
        cidr: cidr_num,
        workers: workers_count,
        duration_ms,
        hosts: active_hosts,
    };

    let json_output = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json_output);
}