use crate::components::Info;
use dioxus::prelude::*;

/// The Home view component.
///
/// It acts as the primary page, rendering the `Info` component
/// and displaying the current application version.
#[component]
pub fn Home() -> Element {
    let pkg_version = env!("CARGO_PKG_VERSION");
    rsx! {
        Info {}
        div { class: "version", "ver: {pkg_version}" }
    }
}
