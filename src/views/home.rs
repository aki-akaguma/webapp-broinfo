use crate::components::Info;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let pkg_version = env!("CARGO_PKG_VERSION");
    rsx! {
        Info {}
        div { class: "version", "ver: {pkg_version}" }
    }
}
