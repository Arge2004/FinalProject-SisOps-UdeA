use std::sync::Arc;

use tokio::sync::Semaphore;

use rust_network_scanner::internal::network::ipcalc::{
    get_local_ip,
    generate_ips
};

use rust_network_scanner::internal::discovery::ping_sweep::{
    discover_host
};

use rust_network_scanner::internal::reporter::console::{
    print_report
};

use rust_network_scanner::internal::reporter::models::{
    HostResult
};

#[tokio::main]
async fn main() {

    println!("Rust Network Scanner");

    let local_ip = get_local_ip();

    println!("IP Local: {}", local_ip);

    let ips = generate_ips(&local_ip);

    let semaphore = Arc::new(
        Semaphore::new(50)
    );

    let mut handles = vec![];

    for ip in ips {

        let semaphore = semaphore.clone();

        let handle = tokio::spawn(async move {

            let _permit =
                semaphore.acquire()
                .await
                .unwrap();

            discover_host(ip).await
        });

        handles.push(handle);
    }

    let mut active_hosts: Vec<HostResult> =
        vec![];

    for handle in handles {

        if let Ok(Some(host)) = handle.await {

            active_hosts.push(host);
        }
    }

    active_hosts.sort_by(|a, b| {
        a.ip.cmp(&b.ip)
    });

    print_report(&active_hosts);

    println!();

    println!(
        "Hosts activos encontrados: {}",
        active_hosts.len()
    );
}