use crate::internal::reporter::models::HostResult;

pub fn print_report(results: &Vec<HostResult>) {

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

        if host.open_ports.is_empty() {

            println!("SIN PUERTOS ABIERTOS");

        } else {

            println!("PUERTOS ABIERTOS:");

            for port in &host.open_ports {

                println!(
                    "  {} -> {}",
                    port.port,
                    port.service
                );
            }
        }

        println!(
            "--------------------------------"
        );
    }
}