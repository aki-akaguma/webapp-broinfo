use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// This is ip address information from `x-forwarded-for`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct AddrInfo {
    /// ip address: ipv4 or ipv6
    pub ip: String,
    /// host name obtained as a result of executing host command
    pub host: String,
}

/// get ip address information
#[post("/api/v1/nasi1", headers: dioxus::fullstack::HeaderMap)]
pub async fn get_address_info(_x: String) -> Result<AddrInfo> {
    let ip = browserinfocm::get_ip_address_string(&headers);
    let host = if ip.is_empty() {
        "".to_string()
    } else {
        command_host(&ip).await?
    };
    Ok(AddrInfo { ip, host })
}

/// invoke host command
#[cfg(feature = "server")]
async fn command_host(ip: &str) -> Result<String> {
    use hickory_resolver::proto::rr::RData;

    // 1. Check if it is correct as an IP address
    let ip_addr: std::net::IpAddr = ip.parse()?;

    // 2. Create a resolver using system default settings
    let resolver = hickory_resolver::Resolver::builder_tokio()?.build()?;

    // 3. Perform a reverse DNS lookup
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
        Err(_) => Ok(String::new()),
    }
}
