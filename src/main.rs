use dioxus::document;
use dioxus::prelude::*;

mod components;
mod pages;
mod server;
mod types;

use components::Navbar;
use pages::{Blog, Home, NotFound, Tools};
use types::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Add comprehensive startup logging
    println!("🚀 Starting Ox server...");
    println!(
        "📍 Current working directory: {:?}",
        std::env::current_dir().unwrap_or_default()
    );
    println!(
        "🌐 PORT: {}",
        std::env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    );
    println!(
        "🔗 IP: {}",
        std::env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string())
    );

    // Log build type
    #[cfg(debug_assertions)]
    println!("🔧 Build: DEBUG");
    #[cfg(not(debug_assertions))]
    println!("🔧 Build: RELEASE");

    // Log feature flags
    println!("⚙️  Features enabled:");
    #[cfg(feature = "web")]
    println!("  - web");
    #[cfg(feature = "server")]
    println!("  - server");
    #[cfg(feature = "fullstack")]
    println!("  - fullstack");

    println!("🎯 Routes configured:");
    println!("  - GET / -> Home");
    println!("  - GET /blog/:id -> Blog");
    println!("  - GET /tools -> Tools");
    println!("  - GET /* -> NotFound (catch-all)");

    println!("📁 Assets configured:");
    println!("  - favicon: {}", FAVICON);
    println!("  - tailwind.css: {}", TAILWIND_CSS);

    println!("🚀 Launching Dioxus application...");

    // Add a simple check to see if we're in server mode
    #[cfg(feature = "server")]
    println!("🔧 Server feature enabled");
    #[cfg(feature = "web")]
    println!("🔧 Web feature enabled");

    #[cfg(feature = "fullstack")]
    {
        println!("🔧 Starting in fullstack mode with server functions enabled");
        dioxus::launch(App);
    }

    #[cfg(not(feature = "fullstack"))]
    {
        println!("🔧 Starting in client-only mode");
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    println!("📱 App component rendering...");
    println!("🔗 Loading assets:");
    println!("  - Favicon: {}", FAVICON);
    println!("  - Tailwind CSS: {}", TAILWIND_CSS);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
