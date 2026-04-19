use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents information about the client's network connection.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct AddrInfo {
    /// The IPv4 or IPv6 address of the client.
    pub ip: String,
    /// The hostname obtained via reverse DNS lookup.
    pub host: String,
}

/// Server function to retrieve the client's IP address and hostname.
///
/// This function extracts the IP from request headers and performs
/// a reverse DNS lookup. It uses tracing to log request context.
#[post("/api/v1/nasi1", headers: dioxus::fullstack::HeaderMap)]
#[tracing::instrument(skip(headers), fields(ip))]
pub async fn get_address_info(_x: String) -> Result<AddrInfo> {
    let ip = browserinfocm::get_ip_address_string(&headers);

    // Record the IP in the current tracing span.
    tracing::Span::current().record("ip", &ip);
    //tracing::info!("Processing address info request");

    let host = if ip.is_empty() {
        "".to_string()
    } else {
        command_host(&ip).await?
    };
    Ok(AddrInfo { ip, host })
}

/// Internal helper to perform a reverse DNS lookup for a given IP.
///
/// Uses `hickory-resolver` to perform an asynchronous PTR record lookup.
#[cfg(feature = "server")]
#[tracing::instrument]
async fn command_host(ip: &str) -> Result<String> {
    use hickory_resolver::proto::rr::RData;

    // Validate if the input is a correct IP address.
    let ip_addr: std::net::IpAddr = ip.parse()?;

    // Create a resolver using system default settings.
    let resolver = hickory_resolver::Resolver::builder_tokio()?.build()?;

    tracing::debug!(target: "dns_lookup", ip = %ip, "Starting reverse DNS lookup");

    // Perform the reverse DNS lookup.
    match resolver.reverse_lookup(ip_addr).await {
        Ok(lookup) => {
            let host = lookup
                .answers()
                .iter()
                .next()
                .map(|rec| match &rec.data {
                    RData::PTR(ptr) => ptr.to_utf8().trim_end_matches('.').to_string(),
                    _ => String::new(),
                })
                .unwrap_or_default();
            Ok(host)
        }
        Err(e) => {
            tracing::warn!(error = %e, "DNS lookup failed, proceeding with empty host");
            Ok(String::new())
        }
    }
}
