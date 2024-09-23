use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::http::HeaderValue;

const DEFAULT_APT_PORT: u16 = 4444;
const DEFAULT_DATABASE_URL: &str = "mongodb://localhost:27017";

/// Get set database url
pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        tracing::warn!(DEFAULT_DATABASE_URL, error = %e,
                "Unable to retrieve DATABASE_URL; falling back to default database url");
        DEFAULT_DATABASE_URL.to_string()
    })
}
/// Get Domain for user
///
/// Returns domain set by env `DOMAIN` If it is unable to retrieve
/// This is the domain that the server is running on
///
/// # Panics
/// This function panics if DOMAIN is not set
pub fn get_domain() -> String {
    std::env::var("DOMAIN").expect("DOMAIN environment variable must be set")
}
/// Get Origin Domain for user
///
/// Returns origin domain set by env `ORIGIN_DOMAIN` If it is unable to retrieve
/// This is the domain that the client is running on
///
/// # Panics
/// This function panics if ORIGIN DOMAIN is not set
pub fn get_origin_domain() -> HeaderValue {
    std::env::var("ORIGIN_DOMAIN")
        .expect("ORIGIN_DOMAIN environment variable must be set")
        .parse::<HeaderValue>()
        .unwrap()
}

pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable must be set")
}

/// Get set api port
///
/// Returns port set by env `API_PORT`. If it is unable to retrieve
///
/// # Panics
/// This function panics if the `DEFAULT_APT_PORT` is not a valid port.
pub fn get_api_port() -> u16 {
    let port = std::env::var("API_PORT").unwrap_or_else(|e| {
        tracing::warn!(DEFAULT_APT_PORT, error = %e,
                "Unable to retrieve API_PORT; falling back to default port");
        DEFAULT_APT_PORT.to_string()
    });

    let port_int = port.parse::<u16>().unwrap_or_else(|e| {
        tracing::error!(DEFAULT_APT_PORT, error = %e, port, "Error in parsing in set port");
        DEFAULT_APT_PORT
    });

    // Checks to see if port given is in range
    if !(1024..=65535).contains(&port_int) {
        tracing::error!(PORT = port_int, "Set port is not a valid port");
        panic!("Set port is not a valid port");
    };

    tracing::info!(PORT = port_int, "Retried api port");
    port_int
}

#[test]
fn returns_valid_port() {
    let port = get_api_port();

    assert!(
        (1024..65535).contains(&port),
        "Given port is not invalid range"
    );
}

/// Gets API address
///
/// Gets the address that the api router should be bound to. This sets the address to `INADDR_ANY`.
/// This is meant for IPv4 api endpoints.
///
/// # Panics
/// Can panic from:
/// - `get_api_port`
pub fn get_api_addr() -> SocketAddr {
    let api_port = get_api_port();

    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), api_port)
}

#[test]
fn returns_valid_api_endpoint() {
    let addr = get_api_addr();

    assert!(addr.is_ipv4());

    assert!(addr.ip().is_unspecified() || addr.ip().is_loopback());
}

pub fn get_metrics_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2222)
}
