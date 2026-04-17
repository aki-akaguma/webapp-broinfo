use browserinfocm::browserinfo::{BroInfo, Browser};
use browserinfocm::BrowserInfoCm;
use dioxus::logger::tracing;
use dioxus::prelude::*;

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

/// the component of browser information
#[component]
pub fn Info() -> Element {
    use crate::backends::AddrInfo;

    // host info
    let mut addrinfo_sig = use_signal(|| None::<AddrInfo>);
    let mut error_msg = use_signal(String::new);
    use_future(move || async move {
        match crate::backends::get_address_info("x".to_string()).await {
            Ok(s) => addrinfo_sig.set(Some(s)),
            Err(e) => {
                tracing::error!("Failed to get address info: {e}");
                error_msg.set(format!("Error: {e}"));
            }
        }
    });

    let addrinfo_content = match &*addrinfo_sig.read() {
        Some(a) => rsx! {
            table {
                tr {
                    th { "IP" }
                    td { class: "row1", "{a.ip}" }
                }
                tr {
                    th { "Host" }
                    td { class: "row2", "{a.host}" }
                }
            }
        },
        None if !error_msg.read().is_empty() => rsx! {
            p { class: "error", "{error_msg}" }
        },
        None => rsx! {
            p { "Loading address information..." }
        },
    };

    // browser info
    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);
    let bicmid_sig = use_signal(String::new);
    let user_sig = use_signal(String::new);

    let bicmid_s = bicmid_sig.read().clone();

    let brg = browser_sig.read().clone();
    let browser_s = format_name_version(&brg.name, &brg.version);
    let os_s = format_os(brg.os.clone());

    let device_s = if !brg.device.is_empty() {
        brg.device.to_string()
    } else {
        String::new()
    };

    let bim = broinfo_sig.read().clone();
    let ua = bim.basic.user_agent;
    let lang = bim.jsinfo.user_language;
    let timezone = bim.jsinfo.timezone;
    let has_cookie = bim.jsinfo.cookie_enabled;
    let has_local_storage = bim.jsinfo.has_local_storage;
    let has_session_storage = bim.jsinfo.has_session_storage;
    let is_dark_mode = bim.jsinfo.is_dark_mode;

    let cores = if let Some(n) = bim.jsinfo.cpu_cores {
        format!("{n}")
    } else {
        String::new()
    };
    let screen_size = if let Some(w) = bim.jsinfo.screen_width {
        if let Some(h) = bim.jsinfo.screen_height {
            format!("{} x {} px", w, h)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    let dpr = if let Some(d) = bim.jsinfo.device_pixel_ratio {
        format!("{d:.2}")
    } else {
        String::new()
    };
    let color_depth = if let Some(n) = bim.jsinfo.screen_color_depth {
        format!("{n}")
    } else {
        String::new()
    };
    let device_memory = if let Some(n) = bim.jsinfo.device_memory {
        format!("{n} GB")
    } else {
        String::new()
    };

    rsx! {
        BrowserInfoCm {
            broinfo: broinfo_sig,
            browser: browser_sig,
            bicmid: bicmid_sig,
            user: user_sig,
        }
        document::Link { rel: "stylesheet", href: INFO_CSS }

        div { class: "info",
            h3 { "Ip Address Information" }
            {addrinfo_content}
        }
        br {}
        div { class: "info",
            h3 { "Browser Information" }
            table {
                InfoRow { label: "Browser", value: browser_s, is_alt: false }
                InfoRow { label: "Language", value: lang, is_alt: true }
                InfoRow {
                    label: "Cookie",
                    value: if has_cookie { "Enable" } else { "Disable" },
                    is_alt: false,
                }
                InfoRow {
                    label: "Session Storage",
                    value: if has_session_storage { "Enable" } else { "Disable" },
                    is_alt: true,
                }
                InfoRow {
                    label: "Local Storage",
                    value: if has_local_storage { "Enable" } else { "Disable" },
                    is_alt: false,
                }
                InfoRow {
                    label: "Dark Mode",
                    value: if is_dark_mode { "Enable" } else { "Disable" },
                    is_alt: true,
                }
                InfoRow { label: "Timezone", value: timezone, is_alt: false }
                InfoRow { label: "User Agent", value: ua, is_alt: true }
            }
        }
        br {}
        div { class: "info",
            h3 { "Hardware Information" }
            table {
                InfoRow { label: "CPU Cores", value: cores, is_alt: true }
                InfoRow { label: "Screen Size", value: screen_size, is_alt: false }
                InfoRow { label: "DPR", value: dpr, is_alt: true }
                InfoRow { label: "Color Depth", value: color_depth, is_alt: false }
                InfoRow { label: "Memory", value: device_memory, is_alt: true }
            }
        }
        br {}
        div { class: "info",
            h3 { "Device Information" }
            table {
                InfoRow { label: "Device", value: device_s, is_alt: true }
                InfoRow { label: "OS", value: os_s, is_alt: false }
            }
        }
        br {}
        div { class: "info",
            h3 { "Bicm Information" }
            table {
                InfoRow { label: "BicmID", value: bicmid_s, is_alt: true }
            }
        }
    }
}

#[component]
fn InfoRow(label: String, value: String, is_alt: bool) -> Element {
    let class = if is_alt { "row2" } else { "row1" };
    rsx! {
        tr {
            th { "{label}" }
            td { class: "{class}", "{value}" }
        }
    }
}

fn format_os(os: Option<browserinfocm::browserinfo::Os>) -> String {
    os.map(|o| format_name_version(&o.name, &o.version))
        .unwrap_or_default()
}

fn format_name_version(name: &str, version: &str) -> String {
    match (name.is_empty(), version.is_empty()) {
        (false, false) => format!("{} {}", name, version),
        (false, true) => name.to_string(),
        _ => String::new(),
    }
}
