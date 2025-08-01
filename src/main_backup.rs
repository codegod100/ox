use dioxus::prelude::*;
use std::fmt;
use base64::{Engine as _, engine::general_purpose};

#[cfg(feature = "server")]
use dioxus::fullstack::prelude::*;

#[derive(Debug, Clone)]
pub struct NoCustomError;

impl fmt::Display for NoCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NoCustomError")
    }
}

impl std::error::Error for NoCustomError {}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/tools")]
    Tools {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
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
    println!("  - GET /* -> NotFound (catch-all)");

    println!("📁 Assets configured:");
    println!("  - favicon: {}", FAVICON);
    println!("  - main.css: {}", MAIN_CSS);
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
    println!("  - Main CSS: {}", MAIN_CSS);
    println!("  - Tailwind CSS: {}", TAILWIND_CSS);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "hero-container",
            div {
                class: "container",
                div {
                    class: "hero-content",
                    h1 {
                        class: "hero-title",
                        "Ox"
                    }
                    p {
                        class: "hero-subtitle",
                        "Modern Rust web applications with Dioxus"
                    }

                    div {
                        class: "cta-buttons",
                        Link {
                            to: Route::Blog { id: 1 },
                            class: "btn btn-primary",
                            "Get Started"
                        }
                    }
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    println!("🏠 Home route matched - rendering Home component");
    rsx! {
        Hero {}
        Features {}
        Echo {}
    }
}

#[component]
fn Features() -> Element {
    rsx! {
        section {
            class: "features-section",
            div {
                class: "container",
                div {
                    class: "features-list",
                    div { class: "feature-item",
                        h3 { "Fast" }
                        p { "Built with Rust for performance" }
                    }
                    div { class: "feature-item",
                        h3 { "Safe" }
                        p { "Type-safe at compile time" }
                    }
                    div { class: "feature-item",
                        h3 { "Full Stack" }
                        p { "Client and server in one codebase" }
                    }
                }
            }
        }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    println!(
        "📝 Blog route matched - rendering Blog component with id: {}",
        id
    );
    let mut cat_url = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut show_confetti = use_signal(|| false);
    let mut blog_content = use_signal(|| (String::new(), String::new()));
    let mut content_loading = use_signal(|| true);

    // Generate blog content when component mounts or ID changes
    use_effect(use_reactive!(|id| {
        spawn(async move {
            content_loading.set(true);
            if let Ok((title, content)) = generate_blog_content(id).await {
                blog_content.set((title, content));
            } else {
                blog_content.set((
                    format!("Blog Post #{}", id),
                    format!("Failed to generate content for blog post #{}. Please try refreshing the page.", id)
                ));
            }
            content_loading.set(false);
        });
    }));

    rsx! {
        div {
            id: "blog",

            // Generated Content
            if content_loading() {
                div { class: "loading", "Generating blog content..." }
            } else {
                div {
                    h1 { "{blog_content().0}" }
                    p { class: "blog-content", "{blog_content().1}" }
                }
            }

            // Cat section
            div {
                class: "cat-section",
                button {
                    class: "btn btn-primary",
                    disabled: loading(),
                    onclick: move |_| async move {
                        loading.set(true);
                        if let Ok(url) = get_random_cat().await {
                            cat_url.set(url);
                        }
                        loading.set(false);
                    },
                    if loading() { "Loading..." } else { "Get Random Cat" }
                }

                if !cat_url().is_empty() {
                    div {
                        class: "cat-display",
                        img {
                            src: "{cat_url}",
                            alt: "Random cat",
                            class: "cat-image",
                            onclick: move |_| {
                                show_confetti.set(true);
                                spawn(async move {
                                    gloo_timers::future::TimeoutFuture::new(2000).await;
                                    show_confetti.set(false);
                                });
                            }
                        }
                    }
                }
            }

            // Secret confetti explosion effect
            if show_confetti() {
                div {
                    class: "confetti-container",
                    for i in 0..25 {
                        {
                            // Generate random explosion parameters
                            let angle = (i as f32 * 14.4 + (i as f32 * 23.7).sin() * 45.0).to_radians();
                            let speed = 200.0 + ((i as f32 * 17.3).sin() * 150.0);
                            let dx = (angle.cos() * speed) as i32;
                            let dy = (angle.sin() * speed) as i32;
                            let rotation = ((i as f32 * 37.1).sin() * 360.0) as i32;

                            rsx! {
                                div {
                                    key: "{i}",
                                    class: format!("confetti-burst confetti-{}", i % 6),
                                    style: format!("--dx: {}; --dy: {}; --rotation: {}deg;",
                                        dx,
                                        dy,
                                        rotation
                                    )
                                }
                            }
                        }
                    }
                }
            }

            // Navigation links
            div {
                class: "blog-nav",
                Link {
                    to: Route::Blog { id: id - 1 },
                    "Previous"
                }
                span { " • " }
                Link {
                    to: Route::Blog { id: id + 1 },
                    "Next"
                }
            }
        }
    }
}

/// Tools page with interactive utilities
#[component]
fn Tools() -> Element {
    println!("🔧 Tools route matched - rendering Tools component");

    let mut active_tab = use_signal(|| {
        let default_tab = "json".to_string();
        println!("🔧 Initializing active_tab with: {}", default_tab);
        default_tab
    });

    let mut is_initialized = use_signal(|| false);

    // Ensure proper initialization on mount with delay to handle WASM loading
    use_effect(move || {
        println!("🔧 Tools component mounted, current tab: {}", active_tab());
        
        // Add small delay to ensure WASM is fully loaded
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(100).await;
            is_initialized.set(true);
            println!("🔧 Tools component fully initialized after WASM load");
        });
    });

    // JSON formatter state
    let mut json_input = use_signal(|| String::new());
    let mut json_output = use_signal(|| String::new());
    let mut json_error = use_signal(|| String::new());

    // Text utilities state
    let mut text_input = use_signal(|| String::new());
    let mut word_count = use_signal(|| 0);
    let mut char_count = use_signal(|| 0);

    // QR code state
    let mut qr_text = use_signal(|| String::new());
    let mut qr_url = use_signal(|| String::new());

    // Base64 state
    let mut base64_input = use_signal(|| String::new());
    let mut base64_output = use_signal(|| String::new());

    let current_tab = active_tab();
    println!("🔧 Tools rendering with active_tab: '{}'", current_tab);

    // Show loading state until fully initialized
    if !is_initialized() {
        return rsx! {
            div {
                class: "min-h-screen pt-24 pb-16 flex items-center justify-center",
                div {
                    class: "text-ctp-subtext0",
                    "Loading tools..."
                }
            }
        };
    }

    rsx! {
        div {
            class: "min-h-screen pt-24 pb-16",
            div {
                class: "container mx-auto px-6 max-w-6xl",
                h1 {
                    class: "text-2xl font-light text-center mb-16 text-ctp-text",
                    "Tools"
                }

                // Tab navigation
                div {
                    class: "flex flex-wrap justify-center gap-4 mb-16",
                button {
                    class: if active_tab() == "json" {
                        "px-6 py-4 bg-ctp-surface1 border border-ctp-surface2 text-ctp-text transition-all duration-200"
                    } else {
                        "px-6 py-4 bg-transparent border border-ctp-surface1 text-ctp-subtext0 hover:text-ctp-text hover:bg-ctp-surface0 transition-all duration-200"
                    },
                    onclick: move |_| {
                        if is_initialized() {
                            println!("🔧 Tab clicked: json");
                            active_tab.set("json".to_string());
                        } else {
                            println!("🔧 Tab click ignored - not initialized yet");
                        }
                    },
                    div {
                        class: "flex flex-col items-center gap-1",
                        div { class: "text-sm font-medium", "JSON" }
                        div { class: "text-xs opacity-75", "format & validate" }
                    }
                }
                button {
                    class: if active_tab() == "text" {
                        "px-6 py-4 bg-ctp-surface1 border border-ctp-surface2 text-ctp-text transition-all duration-200"
                    } else {
                        "px-6 py-4 bg-transparent border border-ctp-surface1 text-ctp-subtext0 hover:text-ctp-text hover:bg-ctp-surface0 transition-all duration-200"
                    },
                    onclick: move |_| {
                        if is_initialized() {
                            println!("🔧 Tab clicked: text");
                            active_tab.set("text".to_string());
                        } else {
                            println!("🔧 Tab click ignored - not initialized yet");
                        }
                    },
                    div {
                        class: "flex flex-col items-center gap-1",
                        div { class: "text-sm font-medium", "Text" }
                        div { class: "text-xs opacity-75", "transform & count" }
                    }
                }
                button {
                    class: if active_tab() == "qr" {
                        "px-6 py-4 bg-ctp-surface1 border border-ctp-surface2 text-ctp-text transition-all duration-200"
                    } else {
                        "px-6 py-4 bg-transparent border border-ctp-surface1 text-ctp-subtext0 hover:text-ctp-text hover:bg-ctp-surface0 transition-all duration-200"
                    },
                    onclick: move |_| {
                        if is_initialized() {
                            println!("🔧 Tab clicked: qr");
                            active_tab.set("qr".to_string());
                        } else {
                            println!("🔧 Tab click ignored - not initialized yet");
                        }
                    },
                    div {
                        class: "flex flex-col items-center gap-1",
                        div { class: "text-sm font-medium", "QR Code" }
                        div { class: "text-xs opacity-75", "generate codes" }
                    }
                }
                button {
                    class: if active_tab() == "base64" {
                        "px-6 py-4 bg-ctp-surface1 border border-ctp-surface2 text-ctp-text transition-all duration-200"
                    } else {
                        "px-6 py-4 bg-transparent border border-ctp-surface1 text-ctp-subtext0 hover:text-ctp-text hover:bg-ctp-surface0 transition-all duration-200"
                    },
                    onclick: move |_| {
                        if is_initialized() {
                            println!("🔧 Tab clicked: base64");
                            active_tab.set("base64".to_string());
                        } else {
                            println!("🔧 Tab click ignored - not initialized yet");
                        }
                    },
                    div {
                        class: "flex flex-col items-center gap-1",
                        div { class: "text-sm font-medium", "Base64" }
                        div { class: "text-xs opacity-75", "encode & decode" }
                    }
                }
            }

                // Tab content
                div {
                    class: "bg-ctp-surface1 border border-ctp-surface2 px-6 py-8 md:px-12 md:py-12 max-w-5xl mx-auto",

                    // Always render content, use match for better control
                    match current_tab.as_str() {
                        "json" => {
                            println!("🔧 Rendering JSON tab");
                            rsx! {
                                JsonFormatter {
                                    input: json_input,
                                    output: json_output,
                                    error: json_error
                                }
                            }
                        }
                        "text" => {
                            println!("🔧 Rendering Text tab");
                            rsx! {
                                TextUtilities {
                                    input: text_input,
                                    word_count: word_count,
                                    char_count: char_count
                                }
                            }
                        }
                        "qr" => {
                            println!("🔧 Rendering QR tab");
                            rsx! {
                                QRGenerator {
                                    input: qr_text,
                                    qr_url: qr_url
                                }
                            }
                        }
                        "base64" => {
                            println!("🔧 Rendering Base64 tab");
                            rsx! {
                                Base64Tool {
                                    input: base64_input,
                                    output: base64_output
                                }
                            }
                        }
                        _ => {
                            println!("🔧 Unknown tab '{}', defaulting to JSON", current_tab);
                            rsx! {
                                JsonFormatter {
                                    input: json_input,
                                    output: json_output,
                                    error: json_error
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// JSON Formatter component
#[component]
fn JsonFormatter(input: Signal<String>, output: Signal<String>, error: Signal<String>) -> Element {
    let mut input = input;
    let mut output = output;
    let mut error = error;

    let mut format_json = move |_| {
        let json_str = input();
        error.set(String::new());

        if json_str.trim().is_empty() {
            output.set(String::new());
            return;
        }

        match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
                Ok(formatted) => output.set(formatted),
                Err(e) => error.set(format!("Formatting error: {}", e)),
            },
            Err(e) => {
                error.set(format!("JSON Parse Error: {}", e));
                output.set(String::new());
            }
        }
    };

    rsx! {
        div {
            class: "space-y-10",

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Input JSON"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text resize-none",
                    rows: "8",
                    placeholder: "Paste your JSON here...",
                    value: "{input}",
                    oninput: move |event| {
                        input.set(event.value());
                        format_json(());
                    }
                }
            }

            // Error display
            if !error().is_empty() {
                div {
                    class: "p-3 bg-ctp-red bg-opacity-20 border border-ctp-red text-ctp-red text-sm",
                    "❌ {error}"
                }
            }

            // Output section
            if !output().is_empty() {
                div {
                    class: "space-y-2",
                    div {
                        class: "flex justify-between items-center",
                        label {
                            class: "block text-sm font-medium text-ctp-subtext1",
                            "Formatted JSON"
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                            onclick: move |_| {
                                println!("📋 Copied JSON to clipboard");
                            },
                            "copy"
                        }
                    }
                    textarea {
                        class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text resize-none",
                        rows: "8",
                        readonly: true,
                        value: "{output}"
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-8 justify-center pt-8",
                button {
                    class: "btn-secondary",
                    onclick: move |_| {
                        input.set(String::new());
                        output.set(String::new());
                        error.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}

/// Text Utilities component
#[component]
fn TextUtilities(
    input: Signal<String>,
    word_count: Signal<i32>,
    char_count: Signal<i32>,
) -> Element {
    let mut input = input;
    let mut word_count = word_count;
    let mut char_count = char_count;

    let mut update_counts = move || {
        let text = input();
        char_count.set(text.len() as i32);
        let words = text.split_whitespace().count();
        word_count.set(words as i32);
    };

    rsx! {
        div {
            class: "space-y-10",

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Input Text"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text resize-none",
                    rows: "8",
                    placeholder: "Enter your text here...",
                    value: "{input}",
                    oninput: move |event| {
                        input.set(event.value());
                        update_counts();
                    }
                }
            }

            // Stats section
            div {
                class: "flex gap-8 justify-center py-3 px-4 bg-ctp-base border border-ctp-surface2",
                div {
                    class: "text-sm text-ctp-subtext1",
                    "Characters: "
                    span { class: "font-medium text-ctp-text", "{char_count}" }
                }
                div {
                    class: "text-sm text-ctp-subtext1",
                    "Words: "
                    span { class: "font-medium text-ctp-text", "{word_count}" }
                }
            }

            // Transform actions
            div {
                class: "space-y-3",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Text Transformations"
                }
                div {
                    class: "flex gap-8 justify-center flex-wrap",
                    button {
                        class: "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            input.set(input().to_uppercase());
                            update_counts();
                        },
                        "UPPERCASE"
                    }
                    button {
                        class: "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            input.set(input().to_lowercase());
                            update_counts();
                        },
                        "lowercase"
                    }
                    button {
                        class: "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            let title_case: String = input()
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars: Vec<char> = word.chars().collect();
                                    if let Some(first) = chars.get_mut(0) {
                                        *first = first.to_uppercase().next().unwrap_or(*first);
                                    }
                                    chars.into_iter().collect::<String>()
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            input.set(title_case);
                            update_counts();
                        },
                        "Title Case"
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-8 justify-center pt-8",
                button {
                    class: "btn-secondary",
                    onclick: move |_| {
                        input.set(String::new());
                        update_counts();
                    },
                    "Clear"
                }
                if !input().is_empty() {
                    button {
                        class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            println!("📋 Copied text to clipboard");
                        },
                        "Copy"
                    }
                }
            }
        }
    }
}

/// QR Code Generator component
#[component]
fn QRGenerator(input: Signal<String>, qr_url: Signal<String>) -> Element {
    let mut input = input;
    let mut qr_url = qr_url;
    let mut loading = use_signal(|| false);

    let generate_qr = move |_| {
        let text = input();
        if text.trim().is_empty() {
            return;
        }

        loading.set(true);
        spawn(async move {
            match generate_qr_code(text).await {
                Ok(url) => qr_url.set(url),
                Err(e) => println!("QR generation error: {:?}", e),
            }
            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "space-y-16",

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Text or URL to encode"
                }
                input {
                    r#type: "text",
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text",
                    placeholder: "Enter text, URL, or any data...",
                    value: "{input}",
                    oninput: move |event| input.set(event.value())
                }
            }

            // Generate button
            div {
                class: "flex justify-center pb-8",
                button {
                    class: if input().trim().is_empty() || loading() {
                        "px-8 py-4 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed transition-colors"
                    } else {
                        "px-8 py-4 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors"
                    },
                    onclick: generate_qr,
                    disabled: input().trim().is_empty() || loading(),
                    if loading() { "Generating..." } else { "Generate QR Code" }
                }
            }

            // Output section
            if !qr_url().is_empty() {
                div {
                    class: "space-y-2",
                    div {
                        class: "flex justify-between items-center",
                        label {
                            class: "block text-sm font-medium text-ctp-subtext1",
                            "Generated QR Code"
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                            onclick: move |_| {
                                println!("📋 Copied QR code URL to clipboard");
                            },
                            "Copy URL"
                        }
                    }
                    div {
                        class: "flex justify-center p-4 bg-ctp-base border border-ctp-surface2",
                        img {
                            src: "{qr_url}",
                            alt: "Generated QR Code",
                            class: "max-w-full h-auto"
                        }
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-8 justify-center pt-12",
                button {
                    class: "btn-secondary",
                    onclick: move |_| {
                        input.set(String::new());
                        qr_url.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}

/// Base64 Encoder/Decoder component
#[component]
fn Base64Tool(input: Signal<String>, output: Signal<String>) -> Element {
    let mut input = input;
    let mut output = output;
    let mut mode = use_signal(|| "encode".to_string());

    let mut process_base64 = move |_| {
        let text = input();
        if text.trim().is_empty() {
            output.set(String::new());
            return;
        }

        if mode() == "encode" {
            let encoded = general_purpose::STANDARD.encode(text.as_bytes());
            output.set(encoded);
        } else {
            match general_purpose::STANDARD.decode(&text) {
                Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                    Ok(decoded_string) => output.set(decoded_string),
                    Err(_) => output.set("Error: Invalid UTF-8 in decoded data".to_string()),
                },
                Err(e) => output.set(format!("Decode error: {}", e)),
            }
        }
    };

    rsx! {
        div {
            class: "space-y-10",

            // Mode selection
            div {
                class: "space-y-3",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Operation Mode"
                }
                div {
                    class: "flex gap-6 justify-center",
                    button {
                        class: if mode() == "encode" {
                            "px-6 py-3 bg-ctp-surface2 text-ctp-text transition-colors"
                        } else {
                            "px-6 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-subtext0 hover:text-ctp-text transition-colors"
                        },
                        onclick: move |_| {
                            mode.set("encode".to_string());
                            process_base64(());
                        },
                        "Encode"
                    }
                    button {
                        class: if mode() == "decode" {
                            "px-6 py-3 bg-ctp-surface2 text-ctp-text transition-colors"
                        } else {
                            "px-6 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-subtext0 hover:text-ctp-text transition-colors"
                        },
                        onclick: move |_| {
                            mode.set("decode".to_string());
                            process_base64(());
                        },
                        "Decode"
                    }
                }
            }

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    if mode() == "encode" { "Text to Encode" } else { "Base64 to Decode" }
                }
                textarea {
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text resize-none",
                    rows: "6",
                    placeholder: if mode() == "encode" {
                        "Enter plain text to convert to Base64..."
                    } else {
                        "Enter Base64 string to decode to plain text..."
                    },
                    value: "{input}",
                    oninput: move |event| {
                        input.set(event.value());
                        process_base64(());
                    }
                }
            }

            // Output section
            if !output().is_empty() {
                div {
                    class: "space-y-2",
                    div {
                        class: "flex justify-between items-center",
                        label {
                            class: "block text-sm font-medium text-ctp-subtext1",
                            if mode() == "encode" { "Base64 Output" } else { "Decoded Text" }
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                            onclick: move |_| {
                                println!("📋 Copied Base64 result to clipboard");
                            },
                            "Copy"
                        }
                    }
                    textarea {
                        class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text resize-none",
                        rows: "6",
                        readonly: true,
                        value: "{output}"
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-8 justify-center pt-8",
                button {
                    class: "btn-secondary",
                    onclick: move |_| {
                        input.set(String::new());
                        output.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}

/// 404 Not Found page - logs unmatched routes
#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    println!("❌ 404 - Route not found: '{}'", path);
    println!("❌ Segments: {:?}", segments);

    rsx! {
        div {
            h1 { "404 - Page Not Found" }
            p { "The requested path '{path}' was not found." }
            p { "Available routes:" }
            ul {
                li { "/" }
                li { "/blog/[id]" }
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    println!("🧭 Navbar component rendering");
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
            Link {
                to: Route::Tools {},
                "Tools"
            }
        }

        Outlet::<Route> {}
    }
}

/// Simple demo of server functions
#[component]
fn Echo() -> Element {
    let mut input = use_signal(|| String::new());
    let mut response = use_signal(|| String::new());

    rsx! {
        section {
            class: "demo-section",
            div {
                class: "container",
                div {
                    class: "demo-card",
                    h3 { "Server Echo" }
                    input {
                        class: "demo-input",
                        placeholder: "Type something...",
                        value: "{input}",
                        oninput: move |event| {
                            input.set(event.value());
                        },
                        onchange: move |_| {
                            let input_val = input();
                            if !input_val.trim().is_empty() {
                                spawn(async move {
                                    println!("🔄 Calling echo_server with: '{}'", input_val);
                                    match echo_server(input_val).await {
                                        Ok(data) => {
                                            println!("✅ Echo success: '{}'", data);
                                            response.set(data);
                                        }
                                        Err(e) => {
                                            println!("❌ Echo error: {:?}", e);
                                            response.set(format!("Error: {}", e));
                                        }
                                    }
                                });
                            } else {
                                response.set(String::new());
                            }
                        },
                    }
                    if !response().is_empty() {
                        div {
                            class: "demo-output",
                            "{response}"
                        }
                    }
                }
            }
        }
    }
}

#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    let start = std::time::Instant::now();
    println!(
        "📡 [{}] Server function called: echo_server with input: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        input
    );

    let response = format!("You said: {}", input);
    let elapsed = start.elapsed();
    println!(
        "📡 [{}] echo_server responding after {:?}: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        elapsed,
        response
    );
    Ok(response)
}

#[server(GetRandomCat)]
async fn get_random_cat() -> Result<String, ServerFnError> {
    println!("📡 Server function called: get_random_cat");
    // Using The Cat API for random cat images
    let response = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    let url = json[0]["url"].as_str().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("No cat URL found".to_string())
    })?;

    Ok(url.to_string())
}

#[server(GenerateBlogContent)]
async fn generate_blog_content(blog_id: i32) -> Result<(String, String), ServerFnError> {
    println!(
        "📡 Server function called: generate_blog_content with blog_id: {}",
        blog_id
    );
    use markov::Chain;

    // Sample corpus for training the Markov chain
    let corpus = "
        Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
        Dioxus is a modern Rust framework for building user interfaces. It provides a declarative way to build cross-platform applications.
        Web development has evolved significantly over the years. Modern frameworks make it easier to build interactive applications.
        Performance optimization is crucial for web applications. Users expect fast loading times and smooth interactions.
        Type safety helps prevent many common programming errors. Rust's ownership system ensures memory safety without garbage collection.
        Full-stack development allows developers to work on both frontend and backend components of an application.
        Server-side rendering improves initial page load times and search engine optimization.
        Component-based architecture promotes code reusability and maintainability.
        Reactive programming paradigms enable building responsive user interfaces.
        Modern tooling and development environments enhance developer productivity.
        Cross-platform development reduces the need to maintain separate codebases.
        API design is important for creating maintainable and scalable applications.
        Database optimization can significantly improve application performance.
        Security considerations are paramount in web application development.
        Code organization and project structure impact long-term maintainability.
        Testing strategies ensure application reliability and catch regressions early.
        Deployment processes should be automated and reliable for production environments.
        User experience design principles guide interface and interaction decisions.
        Accessibility features ensure applications are usable by everyone.
        Documentation helps other developers understand and contribute to projects.
    ";

    // Create and train the Markov chain
    let mut chain = Chain::new();
    chain.feed_str(corpus);

    // Use blog_id to determine which parts of the corpus to start from
    let sentences: Vec<&str> = corpus.split('.').collect();
    let start_idx = (blog_id as usize) % sentences.len();

    // Generate title starting from a specific sentence based on blog_id
    let title_start = sentences.get(start_idx).unwrap_or(&"Rust").trim();
    let title = if !title_start.is_empty() {
        let words: Vec<&str> = title_start.split_whitespace().take(4).collect();
        words.join(" ")
    } else {
        format!("Random Thoughts #{}", blog_id)
    };

    // Generate content using the Markov chain
    let content = if !chain.is_empty() {
        let mut generated = chain.generate_str_from_token("Rust");
        if generated.is_empty() {
            generated = chain.generate_str();
        }

        // Make it longer by appending more generated text
        for _ in 0..2 {
            let more = chain.generate_str();
            if !more.is_empty() {
                generated.push(' ');
                generated.push_str(&more);
            }
        }

        if generated.is_empty() {
            format!("This is a generated blog post using blog ID {} as a seed. The content is created using Markov chains to produce pseudo-random but coherent text about Rust and web development.", blog_id)
        } else {
            generated
        }
    } else {
        format!("This is a generated blog post using blog ID {} as a seed. The content is created using Markov chains to produce pseudo-random but coherent text.", blog_id)
    };

    Ok((title, content))
}

#[server(GenerateQRCode)]
async fn generate_qr_code(text: String) -> Result<String, ServerFnError> {
    let start = std::time::Instant::now();
    println!(
        "📡 [{}] Server function called: generate_qr_code with text: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        text
    );

    if text.trim().is_empty() {
        return Err(ServerFnError::ServerError(
            "Text cannot be empty".to_string(),
        ));
    }

    if text.len() > 2000 {
        return Err(ServerFnError::ServerError(
            "Text too long (max 2000 chars)".to_string(),
        ));
    }

    // Use a QR code generation service API (like qr-server.com)
    let encoded_text = urlencoding::encode(&text);
    let qr_url = format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=200x200&data={}",
        encoded_text
    );

    let elapsed = start.elapsed();
    println!(
        "📡 [{}] generate_qr_code responding after {:?} with URL: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        elapsed,
        qr_url
    );
    Ok(qr_url)
}
