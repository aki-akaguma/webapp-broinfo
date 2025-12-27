use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_process::Command;

/// This is information ``x-forwarded-for`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct AddrInfo {
    pub ip: String,
    pub host: String,
}

#[post("/api/v1/nasi1", headers: dioxus::fullstack::HeaderMap)]
pub async fn get_address_info() -> Result<AddrInfo> {
    /*
    let ip = if let Some(s) = headers.get("x-forwarded-for") {
        // format:
        //     X-Forwarded-For: client1, proxy1, proxy2, ...
        let ss = s.to_str().unwrap();
        if let Some(idx) = ss.find(',') {
            ss[..idx].to_string()
        } else {
            ss.to_string()
        }
    } else {
        "".to_string()
    };
    */
    let ip = browserinfocm::get_ipaddress_string(&headers);
    let host = if ip.is_empty() {
        "".to_string()
    } else {
        command_host(&ip).await?
    };
    Ok(AddrInfo { ip, host })
}

#[cfg(feature = "server")]
async fn command_host(ip: &str) -> Result<String> {
    let out = Command::new("host").arg(&ip).output().await?;
    let r = if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout).to_string();
        let ss = s.as_str();
        // 154.116.168.192.in-addr.arpa domain name pointer aki-hcc2.
        // 154.116.168.192.in-addr.arpa domain name pointer aki-hcc2.local.
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
