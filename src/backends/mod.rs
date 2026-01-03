use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_process::Command;

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
    let ip = browserinfocm::get_ipaddress_string(&headers);
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
    let out = Command::new("host").arg(&ip).output().await?;
    let r = if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout).to_string();
        let ss = s.as_str();
        // 154.116.168.192.in-addr.arpa domain name pointer aki-dst2.
        // 154.116.168.192.in-addr.arpa domain name pointer aki-dst2.local.
        let pat_s = " domain name pointer ";
        if let Some(idx) = ss.rfind(pat_s) {
            let sss = &ss[idx + pat_s.len()..].trim();
            if let Some(ssss) = sss.strip_suffix(".") {
                ssss.to_string()
            } else {
                sss.to_string()
            }
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    Ok(r)
}
