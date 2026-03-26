use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub log_filter: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            service_name: read_env("APP_SERVICE_NAME", "hello_rust"),
            host: read_env("APP_HOST", "127.0.0.1"),
            port: read_env_parse("APP_PORT", 3000_u16),
            log_filter: read_env("RUST_LOG", "info,hello_rust=debug"),
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        let parsed_ip = self.host.parse::<std::net::IpAddr>().unwrap_or_else(|_| {
            panic!(
                "APP_HOST must be a valid IP address, current value: {}",
                self.host
            )
        });

        SocketAddr::from((parsed_ip, self.port))
    }
}

fn read_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn read_env_parse<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}
