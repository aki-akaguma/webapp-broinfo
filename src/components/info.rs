use browserinfocm::browserinfo::{BroInfo, Browser};
use browserinfocm::BrowserInfoCm;
use dioxus::prelude::*;

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

#[component]
pub fn Info() -> Element {
    use crate::backends::AddrInfo;

    // host info
    let mut addrinfo_sig = use_signal(AddrInfo::default);
    use_future(move || async move {
        //let s = dioxus::fullstack::get_server_url().to_string();
        //info!("server url: '{s}'");
        let s = crate::backends::get_address_info().await.unwrap();
        addrinfo_sig.set(s);
    });
    let a = addrinfo_sig.read().clone();
    let addrinfo_ip_s = a.ip;
    let addrinfo_host_s = a.host;

    // browser info
    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);
    let user_sig = use_signal(String::new);

    let brg = browser_sig.read().clone();
    let browser_s = if !brg.name.is_empty() && !brg.version.is_empty() {
        format!("{} (ver. {})", brg.name, brg.version)
    } else if !brg.name.is_empty() {
        brg.name.to_string()
    } else {
        String::new()
    };
    let os_s = if let Some(o) = brg.os {
        if !o.name.is_empty() && !o.version.is_empty() {
            format!("{} {}", o.name, o.version)
        } else if !o.name.is_empty() {
            o.name.to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
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
    let dpr = if let Some(d) = bim.jsinfo.device_pixcel_ratio {
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
        BrowserInfoCm { broinfo: broinfo_sig, browser: browser_sig, user: user_sig }
        document::Link { rel: "stylesheet", href: INFO_CSS }

        div { class: "info",
            h3 { "Ip Address Information" }
            table {
                tr {
                    th { "IP" }
                    td { class: "row1", "{addrinfo_ip_s}" }
                }
                tr {
                    th { "Host" }
                    td { class: "row2", "{addrinfo_host_s}" }
                }
            }
        }
        br {}
        div { class: "info",
            h3 { "Browser Information" }
            table {
                tr {
                    th { "Browser" }
                    td { class: "row1", "{browser_s}" }
                }
                tr {
                    th { "Language" }
                    td { class: "row2", "{lang}" }
                }
                tr {
                    th { "Cookie" }
                    td { class: "row2",
                        if has_cookie {
                            "Enable"
                        } else {
                            "Disable"
                        }
                    }
                }
                tr {
                    th { "Session Storage" }
                    td { class: "row1",
                        if has_session_storage {
                            "Enable"
                        } else {
                            "Disable"
                        }
                    }
                }
                tr {
                    th { "Local Storage" }
                    td { class: "row2",
                        if has_local_storage {
                            "Enable"
                        } else {
                            "Disable"
                        }
                    }
                }
                tr {
                    th { "Dark Mode" }
                    td { class: "row1",
                        if is_dark_mode {
                            "Enable"
                        } else {
                            "Disable"
                        }
                    }
                }
                tr {
                    th { "Timezone" }
                    td { class: "row2", "{timezone}" }
                }
                tr {
                    th { "User Agent" }
                    td { class: "row1", "{ua}" }
                }
            }
        }
        br {}
        div { class: "info",
            h3 { "Hardware Information" }
            table {
                tr {
                    th { "CPU Cores" }
                    td { class: "row1", "{cores}" }
                }
                tr {
                    th { "Screen Size" }
                    td { class: "row2", "{screen_size}" }
                }
                tr {
                    th { "DPR" }
                    td { class: "row1", "{dpr}" }
                }
                tr {
                    th { "Color Depth" }
                    td { class: "row2", "{color_depth}" }
                }
                tr {
                    th { "Memory" }
                    td { class: "row1", "{device_memory}" }
                }
            }
        }
        br {}
        div { class: "info",
            h3 { "Device Information" }
            table {
                tr {
                    th { "Device" }
                    td { class: "row1", "{device_s}" }
                }
                tr {
                    th { "OS" }
                    td { class: "row2", "{os_s}" }
                }
            }
        }
    }
}
