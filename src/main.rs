use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

use views::Home;

mod backends;
mod components;
mod views;

/// The main entry point of the application.
///
/// It initializes the logger, sets up platform-specific configurations
/// (Web, Desktop, or Mobile), and launches the Dioxus app.
fn main() {
    // Supplement panic hook on Firefox/Web browsers for better debugging.
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    // Initialize the logger with appropriate levels based on build type.
    #[cfg(not(debug_assertions))]
    let level = dioxus::logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus::logger::tracing::Level::DEBUG;
    dioxus::logger::init(level).expect("failed to init logger");

    // Connect Desktop/Mobile release builds to the public backend API.
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        // Specify the URL of the deployed public webapp.
        let backend_url = "https://aki.omusubi.org/broinfo";
        dioxus_fullstack::set_server_url(backend_url);
    }

    #[cfg(feature = "server")]
    browserinfocm::backend_init().expect("failed to init backend");

    // Launch configuration for Desktop apps (release mode).
    #[cfg(all(not(debug_assertions), feature = "desktop"))]
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Browser Information"),
            ),
        )
        .launch(App);

    // Standard launch for other targets (Web, Mobile, or Debug Desktop).
    #[cfg(any(debug_assertions, not(feature = "desktop")))]
    dioxus::launch(App);
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

/// The root component of the application.
///
/// It manages global document links (fonts, icons) and renders
/// the main style and the Home view.
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Bagel+Fat+One:wght@400&display=swap",
        }
        document::Link { rel: "icon", href: FAVICON }

        MyStyle {}
        Home {}
    }
}

/// A component that injects the main CSS stylesheet.
///
/// Depending on the `inline_style` feature, it either links to an
/// external file or inlines minified CSS directly into the document.
#[cfg(not(feature = "inline_style"))]
#[component]
fn MyStyle() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/main.css") }
    }
}

#[cfg(feature = "inline_style")]
#[component]
fn MyStyle() -> Element {
    const MAIN_CSS: &str = const_css_minify::minify!("../assets/styling/main.css");
    rsx! {
        style { "{MAIN_CSS}" }
    }
}
