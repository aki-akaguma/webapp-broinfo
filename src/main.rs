use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

use views::Home;

mod backends;
mod components;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    // you can set the ports and IP manually with env vars:
    // server launch:
    // IP="0.0.0.0" PORT=8080 ./server

    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    /*
    #[cfg(not(feature = "web"))]
    #[cfg(any(
        not(debug_assertions),
        not(feature = "desktop"),
        not(feature = "server")
    ))]
    {
        //let backend_url = "https://aki.omusubi.org/broinfo";
        let backend_url = "https://aki.omusubi.org";
        dioxus_fullstack::set_server_url(backend_url);
    }
    */
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        let backend_url = "https://aki.omusubi.org/broinfo";
        //let backend_url = "http://aki.omusubi.org/broinfo";
        dioxus_fullstack::set_server_url(backend_url);
    }

    /*
    let s = browserinfo::browserinfo_s();
    dioxus_logger::tracing::info!("{s}");
    */

    #[cfg(any(debug_assertions, not(feature = "desktop")))]
    dioxus::launch(App);

    #[cfg(all(not(debug_assertions), feature = "desktop"))]
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Tap tap tap beat"),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
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

        MyStyle {}
        Home {}
    }
}

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
