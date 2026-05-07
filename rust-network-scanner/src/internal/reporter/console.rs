use crate::internal::reporter::models::{
    HostResult,
    PortResult
};

pub fn print_report(
    results: &Vec<HostResult>
) {

    println!();

    println!(
        "========== RESULTADOS =========="
    );

    for host in results {

        println!();

        println!(
            "HOST: {}",
            host.ip
        );

        println!(
            "LATENCIA: {} ms",
            host.latency
        );

        // TCP
        println!();

        println!("TCP:");

        if host
            .scan_result
            .tcp_ports
            .is_empty() {

            println!(
                "  SIN PUERTOS TCP ABIERTOS"
            );

        } else {

            for port in
                &host
                .scan_result
                .tcp_ports {

                print_port(port);
            }
        }

        // UDP
        println!();

        println!("UDP:");

        if host
            .scan_result
            .udp_ports
            .is_empty() {

            println!(
                "  SIN PUERTOS UDP ABIERTOS"
            );

        } else {

            for port in
                &host
                .scan_result
                .udp_ports {

                print_port(port);
            }
        }

        println!(
            "--------------------------------"
        );
    }
}

fn print_port(
    port: &PortResult
) {
    println!(
        "  {} -> {}",
        port.port,
        port.service
    );
}