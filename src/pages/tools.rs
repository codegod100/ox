use crate::components::tools::*;
use dioxus::prelude::*;

/// Tools page with interactive utilities
#[component]
pub fn Tools() -> Element {
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

    // Tool state
    let mut json_input = use_signal(|| String::new());
    let mut json_output = use_signal(|| String::new());
    let mut json_error = use_signal(|| String::new());

    let mut text_input = use_signal(|| String::new());
    let mut word_count = use_signal(|| 0);
    let mut char_count = use_signal(|| 0);

    let mut qr_text = use_signal(|| String::new());
    let mut qr_url = use_signal(|| String::new());

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
