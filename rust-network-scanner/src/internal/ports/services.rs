use std::collections::HashMap;

pub fn get_service_name(
    port: u16
) -> String {

    let services: HashMap<u16, &str> =
        HashMap::from([

        (20, "FTP-DATA"),
        (21, "FTP"),
        (22, "SSH"),
        (23, "TELNET"),
        (25, "SMTP"),
        (53, "DNS"),
        (67, "DHCP"),
        (68, "DHCP"),
        (69, "TFTP"),
        (80, "HTTP"),
        (88, "KERBEROS"),
        (110, "POP3"),
        (111, "RPCBIND"),
        (123, "NTP"),
        (135, "RPC"),
        (137, "NETBIOS-NS"),
        (138, "NETBIOS-DGM"),
        (139, "NETBIOS-SSN"),
        (143, "IMAP"),
        (161, "SNMP"),
        (389, "LDAP"),
        (443, "HTTPS"),
        (445, "SMB"),
        (465, "SMTPS"),
        (514, "SYSLOG"),
        (587, "SMTP Submission"),
        (631, "IPP"),
        (636, "LDAPS"),
        (873, "RSYNC"),
        (993, "IMAPS"),
        (995, "POP3S"),
        (1025, "NFS/IIS"),
        (1080, "SOCKS"),
        (1194, "OPENVPN"),
        (1433, "MSSQL"),
        (1521, "ORACLE"),
        (1723, "PPTP"),
        (1883, "MQTT"),
        (2049, "NFS"),
        (2082, "CPANEL"),
        (2083, "CPANEL SSL"),
        (2375, "DOCKER"),
        (2376, "DOCKER TLS"),
        (2483, "ORACLE DB"),
        (3000, "NODEJS"),
        (3128, "SQUID"),
        (3306, "MYSQL"),
        (3389, "RDP"),
        (3690, "SVN"),
        (4000, "ICQ"),
        (4040, "SPARK"),
        (4369, "ERLANG"),
        (5000, "UPNP/FLASK"),
        (5060, "SIP"),
        (5432, "POSTGRESQL"),
        (5601, "KIBANA"),
        (5672, "RABBITMQ"),
        (5900, "VNC"),
        (5984, "COUCHDB"),
        (6379, "REDIS"),
        (6443, "KUBERNETES"),
        (6667, "IRC"),
        (7001, "WEBLOGIC"),
        (8080, "HTTP-ALT"),
        (8081, "HTTP-ALT"),
        (8086, "INFLUXDB"),
        (8088, "HADOOP"),
        (8090, "JENKINS"),
        (8443, "HTTPS-ALT"),
        (8888, "JUPYTER"),
        (9000, "SONARQUBE"),
        (9092, "KAFKA"),
        (9200, "ELASTICSEARCH"),
        (9418, "GIT"),
        (11211, "MEMCACHED"),
        (15672, "RABBITMQ WEB"),
        (27017, "MONGODB"),
        (33060, "MYSQL X"),
        (50070, "HDFS"),
    ]);

    services
        .get(&port)
        .unwrap_or(&"UNKNOWN")
        .to_string()
}

pub fn get_udp_service_name(
    port: u16
) -> String {

    let services: HashMap<u16, &str> =
        HashMap::from([

        // CORE
        (53, "DNS"),
        (67, "DHCP SERVER"),
        (68, "DHCP CLIENT"),
        (69, "TFTP"),
        (80, "HTTP/UDP"),
        (88, "KERBEROS"),
        (111, "RPCBIND"),
        (119, "NNTP"),
        (123, "NTP"),
        (135, "RPC"),
        (137, "NETBIOS-NS"),
        (138, "NETBIOS-DGM"),
        (139, "NETBIOS-SSN"),
        (143, "IMAP"),
        (161, "SNMP"),
        (162, "SNMPTRAP"),
        (177, "XDMCP"),
        (389, "LDAP"),
        (427, "SLP"),
        (443, "HTTPS/UDP"),
        (445, "SMB"),
        (500, "IKE/IPSEC"),
        (514, "SYSLOG"),
        (520, "RIP"),
        (631, "IPP"),
        (996, "VSINET"),
        (998, "PUAKMA"),
        (999, "APPLIX"),
        (1194, "OPENVPN"),
        (1434, "MSSQL MONITOR"),
        (1701, "L2TP"),
        (1812, "RADIUS"),
        (1813, "RADIUS ACCOUNTING"),
        (1900, "SSDP"),
        (2049, "NFS"),
        (2083, "CPANEL SSL"),
        (2222, "DIRECTADMIN"),
        (3283, "APPLE REMOTE"),
        (3306, "MYSQL"),
        (3389, "RDP"),
        (3478, "STUN"),
        (3702, "WS-DISCOVERY"),
        (4500, "IPSEC NAT-T"),
        (5000, "UPNP"),
        (5060, "SIP"),
        (5061, "SIP TLS"),
        (5353, "MDNS"),
        (5432, "POSTGRESQL"),
        (5500, "VNC"),
        (5601, "KIBANA"),
        (5683, "COAP"),
        (5900, "VNC"),
        (6000, "X11"),
        (6379, "REDIS"),
        (7000, "AFS"),
        (7070, "REALSERVER"),
        (8080, "HTTP ALT"),
        (8086, "INFLUXDB"),
        (8443, "HTTPS ALT"),
        (8888, "JUPYTER"),
        (9200, "ELASTICSEARCH"),
        (11211, "MEMCACHED"),
        (27017, "MONGODB"),

        // GAMING
        (27015, "STEAM"),
        (25565, "MINECRAFT"),
        (19132, "MINECRAFT BEDROCK"),
        (3074, "XBOX LIVE"),

        // VOIP
        (3479, "STUN ALT"),
        (5349, "TURN TLS"),

        // VPN
        (51820, "WIREGUARD"),

        // MULTICAST
        (5355, "LLMNR"),

        // CONTAINERS / CLOUD
        (6443, "KUBERNETES"),
        (2375, "DOCKER"),
        (2376, "DOCKER TLS"),

        // MONITORING
        (8125, "STATSD"),
        (9090, "PROMETHEUS"),
    ]);

    services
        .get(&port)
        .unwrap_or(&"UNKNOWN")
        .to_string()
}