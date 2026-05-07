use local_ip_address::local_ip;

pub fn get_local_ip() -> String {

    match local_ip() {

        Ok(ip) => ip.to_string(),

        Err(_) => {
            panic!("No se pudo obtener la IP local");
        }
    }
}

pub fn generate_ips(local_ip: &str) -> Vec<String> {

    let parts: Vec<&str> = local_ip.split('.').collect();

    if parts.len() != 4 {
        panic!("IP inválida");
    }

    let subnet = format!("{}.{}.{}", parts[0], parts[1], parts[2]);

    let mut ips = Vec::new();

    for i in 1..255 {
        ips.push(format!("{}.{}", subnet, i));
    }

    ips
}