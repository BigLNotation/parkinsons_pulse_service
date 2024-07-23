use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const DEFAULT_APT_PORT: u16 = 4444;

/// Get set api port
///
/// Returns port set by env "API_PORT". If it is unable to retrieve
///
/// # Panics
/// This function panics if the DEFAULT_APT_PORT is not a valid port.
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
/// Gets the address that the api router should be bound to. This sets the address to INADDR_ANY.
///
/// # Panics
/// Can panic from:
/// - get_api_port
pub fn get_api_addr() -> SocketAddr {
    let api_port = get_api_port();

    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), api_port)
}
