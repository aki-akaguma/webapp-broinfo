use browserinfocm::browserinfo::BroInfo;
use browserinfocm::BrowserInfoCm;
use browserinfocm::BrowserInfoState;
use dioxus::logger::tracing;
use dioxus::prelude::*;

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

use crate::backends::AddrInfo;

/// Main component that displays comprehensive browser and hardware information.
///
/// It coordinates data fetching from the backend (IP/Host) and gathers
/// client-side metrics using the `BrowserInfoCm` component.
#[component]
pub fn Info() -> Element {
    // Signal to trigger retries for network data fetching.
    let mut retry_count_sig = use_signal(|| 0);
    // Consolidated state for network information.
    let mut addr_state_sig = use_signal(|| AddressState::Loading);

    use_future(move || async move {
        // Dependencies: re-runs when retry_count_sig changes.
        let _ = retry_count_sig.read();
        addr_state_sig.set(AddressState::Loading);

        match crate::backends::get_address_info("x".to_string()).await {
            Ok(info) => addr_state_sig.set(AddressState::Success(info)),
            Err(e) => {
                tracing::error!("Failed to get address info: {e}");
                addr_state_sig.set(AddressState::Error(format!("Error: {e}")));
            }
        }
    });

    // Determine the UI content for the IP Information section based on fetch state.
    let addrinfo_content = match &*addr_state_sig.read() {
        AddressState::Loading => rsx! {
            div { class: "loading-box", "Loading address information..." }
        },
        AddressState::Error(err_msg) => {
            let (title, desc) = friendly_error(err_msg);
            rsx! {
                div { class: "error-card",
                    h4 { "{title}" }
                    p { "{desc}" }
                    details {
                        summary { "Detailed error information (for developers)" }
                        pre { "{err_msg}" }
                    }
                    button { onclick: move |_| retry_count_sig += 1, "Try Again" }
                }
            }
        }
        AddressState::Success(a) => rsx! {
            table {
                InfoRow { label: "IP", value: a.ip.clone(), is_alt: false }
                InfoRow { label: "Host", value: a.host.clone(), is_alt: true }
            }
        },
    };

    // Signals for storing data gathered by BrowserInfoCm.
    let state_sig = use_signal(BrowserInfoState::default);

    // Format bicmid
    let bicmid_s = &state_sig.read().bicmid;

    // Format basic browser and OS information.
    let brg = &state_sig.read().browser;
    let browser_s = format_name_version(&brg.name, &brg.version);
    let os_s = format_os(brg.os.clone());

    let device_s = if !brg.device.is_empty() {
        brg.device.to_string()
    } else {
        String::new()
    };

    // Consolidate raw data into display-friendly structures.
    let hw = HardwareDisplayInfo::from_broinfo(&state_sig.read().broinfo);
    let bro = BrowserDisplayInfo::from_broinfo(&state_sig.read().broinfo);

    rsx! {
        // Background component for gathering browser info via JS interop.
        BrowserInfoCm { state: state_sig }
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
                InfoRow { label: "Language", value: bro.lang, is_alt: true }
                InfoRow {
                    label: "Cookie",
                    value: if bro.has_cookie { "Enable" } else { "Disable" },
                    is_alt: false,
                }
                InfoRow {
                    label: "Session Storage",
                    value: if bro.has_session_storage { "Enable" } else { "Disable" },
                    is_alt: true,
                }
                InfoRow {
                    label: "Local Storage",
                    value: if bro.has_local_storage { "Enable" } else { "Disable" },
                    is_alt: false,
                }
                InfoRow {
                    label: "Dark Mode",
                    value: if bro.is_dark_mode { "Enable" } else { "Disable" },
                    is_alt: true,
                }
                InfoRow { label: "Timezone", value: bro.timezone, is_alt: false }
                InfoRow { label: "User Agent", value: bro.ua, is_alt: true }
            }
        }
        br {}
        div { class: "info",
            h3 { "Hardware Information" }
            table {
                InfoRow { label: "CPU Cores", value: hw.cores, is_alt: true }
                InfoRow {
                    label: "Screen Size",
                    value: hw.screen_size,
                    is_alt: false,
                }
                InfoRow { label: "DPR", value: hw.dpr, is_alt: true }
                InfoRow {
                    label: "Color Depth",
                    value: hw.color_depth,
                    is_alt: false,
                }
                InfoRow { label: "Memory", value: hw.memory, is_alt: true }
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

/// A helper component that renders a single labeled row in a table.
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

/// Represents the state of the backend address information fetch.
#[derive(Clone, PartialEq)]
enum AddressState {
    /// Request is in progress.
    Loading,
    /// Request succeeded with data.
    Success(AddrInfo),
    /// Request failed with an error message.
    Error(String),
}

/// Formatted browser-related information for display.
struct BrowserDisplayInfo {
    ua: String,
    lang: String,
    timezone: String,
    has_cookie: bool,
    has_local_storage: bool,
    has_session_storage: bool,
    is_dark_mode: bool,
}

impl BrowserDisplayInfo {
    /// Extracts and formats browser info from raw `BroInfo`.
    fn from_broinfo(bim: &BroInfo) -> Self {
        Self {
            ua: bim.basic.user_agent.to_string(),
            lang: bim.jsinfo.user_language.to_string(),
            timezone: bim.jsinfo.timezone.to_string(),
            has_cookie: bim.jsinfo.cookie_enabled,
            has_local_storage: bim.jsinfo.has_local_storage,
            has_session_storage: bim.jsinfo.has_session_storage,
            is_dark_mode: bim.jsinfo.is_dark_mode,
        }
    }
}

/// Formatted hardware-related information for display.
struct HardwareDisplayInfo {
    cores: HardwareValue,
    screen_size: HardwareValue,
    dpr: HardwareValue,
    color_depth: HardwareValue,
    memory: HardwareValue,
}

impl HardwareDisplayInfo {
    /// Extracts and formats hardware metrics from raw `BroInfo`.
    fn from_broinfo(bim: &BroInfo) -> Self {
        let js = &bim.jsinfo;
        Self {
            cores: HardwareValue::Cores(js.cpu_cores.unwrap_or_default()),
            screen_size: match (js.screen_width, js.screen_height) {
                (Some(width), Some(height)) => HardwareValue::Pixels { width, height },
                _ => HardwareValue::None,
            },
            dpr: HardwareValue::Dpr(js.device_pixel_ratio.unwrap_or_default()),
            color_depth: HardwareValue::Bits(js.screen_color_depth.unwrap_or_default()),
            memory: HardwareValue::MemoryGB(js.device_memory.unwrap_or_default()),
        }
    }
}

/// Representing hardware numbers with units
enum HardwareValue {
    Cores(i32),
    Pixels { width: i32, height: i32 },
    Dpr(f64),
    Bits(i32),
    MemoryGB(i32),
    None,
}

impl std::fmt::Display for HardwareValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cores(n) => write!(f, "{n}"),
            Self::Pixels { width, height } => write!(f, "{width} x {height} px"),
            Self::Dpr(d) => write!(f, "{d:.2}"),
            Self::Bits(n) => write!(f, "{n} bits"),
            Self::MemoryGB(n) => {
                if *n > 0 {
                    write!(f, "{n} GB")
                } else {
                    write!(f, "")
                }
            }
            Self::None => write!(f, "-"),
        }
    }
}

/// Helper to format OS name and version.
fn format_os(os: Option<browserinfocm::browserinfo::Os>) -> String {
    os.map(|o| format_name_version(&o.name, &o.version))
        .unwrap_or_default()
}

/// Combines name and version into a single string (e.g., "Chrome 120").
fn format_name_version(name: &str, version: &str) -> String {
    match (name.is_empty(), version.is_empty()) {
        (false, false) => format!("{} {}", name, version),
        (false, true) => name.to_string(),
        _ => String::new(),
    }
}

/// Translates raw error messages into user-friendly titles and descriptions.
fn friendly_error(raw_error: &str) -> (&str, &str) {
    if raw_error.contains("timeout") {
        (
            "Connection Timeout",
            "Please check your network status and try again.",
        )
    } else if raw_error.contains("refused") || raw_error.contains("404") {
        (
            "Unable to connect to server",
            "The server may be undergoing maintenance or is temporarily unavailable.",
        )
    } else {
        (
            "An error has occurred",
            "An unexpected error occurred. Please try again later.",
        )
    }
}
