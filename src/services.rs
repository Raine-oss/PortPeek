// Services

pub fn get_service_hint(port: u16, no_hints: bool) -> Option<&'static str> {
    if no_hints {
        return None;
    }

    match port {
        20 => Some("FTP Data"),
        21 => Some("FTP"),
        22 => Some("SSH"),
        23 => Some("Telnet"),
        25 => Some("SMTP"),
        53 => Some("DNS"),
        80 => Some("HTTP"),
        110 => Some("POP3"),
        123 => Some("NTP"),
        143 => Some("IMAP"),
        443 => Some("HTTPS"),
        465 => Some("SMTPS"),
        587 => Some("SMTP Submission"),
        993 => Some("IMAPS"),
        995 => Some("POP3S"),
        1433 => Some("MSSQL"),
        1521 => Some("Oracle DB"),
        2375 => Some("Docker"),
        3000 => Some("Dev Server"),
        3306 => Some("MySQL"),
        5000 => Some("Dev Server"),
        5432 => Some("PostgreSQL"),
        5672 => Some("RabbitMQ"),
        6379 => Some("Redis"),
        8000 => Some("Dev Server"),
        8080 => Some("HTTP Proxy / Dev"),
        8081 => Some("Dev Server"),
        8443 => Some("HTTPS Alt"),
        9000 => Some("Dev Server"),
        9092 => Some("Kafka"),
        9200 => Some("Elasticsearch"),
        11211 => Some("Memcached"),
        25565 => Some("Minecraft Java"),
        27017 => Some("MongoDB"),
        _ => None,
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_services() {
        assert_eq!(get_service_hint(22, false), Some("SSH"));
        assert_eq!(get_service_hint(80, false), Some("HTTP"));
        assert_eq!(get_service_hint(443, false), Some("HTTPS"));
        assert_eq!(get_service_hint(25565, false), Some("Minecraft Java"));
        assert_eq!(get_service_hint(9200, false), Some("Elasticsearch"));
    }

    #[test]
    fn test_unknown_service() {
        assert_eq!(get_service_hint(59999, false), None);
    }

    #[test]
    fn test_no_hints_flag() {
        assert_eq!(get_service_hint(22, true), None);
        assert_eq!(get_service_hint(80, true), None);
    }
}
