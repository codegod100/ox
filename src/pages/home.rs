use crate::server::echo_server;
use crate::types::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    println!("🏠 Home route matched - rendering Home component");
    rsx! {
        div { class: "min-h-screen bg-ctp-base",
            Hero {}
            Features {}
            Echo {}
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "flex items-center justify-center min-h-screen px-6 pt-32",
            div { class: "text-center max-w-2xl mx-auto animate-fade-in",
                h1 { class: "text-6xl md:text-7xl font-light text-ctp-mauve mb-6 tracking-wide",
                    "Ox"
                }
                p { class: "text-xl text-ctp-subtext0/70 mb-12 leading-relaxed",
                    "Modern Rust web applications with Dioxus"
                }
                Link {
                    to: Route::Blog { id: 1 },
                    class: "inline-block px-8 py-3 border-2 border-ctp-mauve text-ctp-mauve hover:bg-ctp-mauve hover:text-ctp-base bg-transparent transition-colors rounded-md font-medium",
                    "Get Started"
                }
            }
        }
    }
}

#[component]
pub fn Features() -> Element {
    rsx! {
        section { class: "py-20 px-6",
            div { class: "container mx-auto max-w-4xl",
                h2 { class: "text-3xl font-light text-ctp-text text-center mb-16",
                    "Built for Excellence"
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    FeatureCard {
                        icon: "🛡️",
                        title: "Type Safe",
                        description: "Built with Rust's type system for reliability",
                    }
                    FeatureCard {
                        icon: "⚡",
                        title: "Fast",
                        description: "Optimized for performance and low latency",
                    }
                    FeatureCard {
                        icon: "🔗",
                        title: "Full Stack",
                        description: "Server and client code in one codebase",
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-lg p-8 text-center hover:bg-ctp-surface1/50 transition-colors",
            div { class: "text-4xl mb-4", "{icon}" }
            h3 { class: "text-xl font-medium text-ctp-text mb-3", "{title}" }
            p { class: "text-ctp-subtext0/70 leading-relaxed", "{description}" }
        }
    }
}

/// Simple demo of server functions
#[component]
fn Echo() -> Element {
    let mut input = use_signal(|| String::new());
    let mut response = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut request_id = use_signal(|| 0u32);

    rsx! {
        section { class: "py-20 px-6",
            div { class: "container mx-auto max-w-lg",
                div { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-lg p-8",
                    h3 { class: "text-xl font-medium text-ctp-text text-center mb-8",
                        "Real-time Server Echo"
                    }
                    p { class: "text-sm text-ctp-subtext0 text-center mb-6",
                        "Type below and see the server respond in real-time with debouncing"
                    }
                    div { class: "space-y-4",
                        input {
                            class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 rounded-md text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-mauve transition-colors",
                            placeholder: "Start typing...",
                            value: "{input}",
                            oninput: move |event| {
                                let input_val = event.value();
                                input.set(input_val.clone());
                                
                                let current_request_id = request_id() + 1;
                                request_id.set(current_request_id);
                                
                                if !input_val.trim().is_empty() {
                                    is_loading.set(true);
                                    spawn(async move {
                                        gloo_timers::future::TimeoutFuture::new(500).await;
                                        
                                        if request_id() == current_request_id {
                                            println!("🔄 Real-time calling echo_server with: '{}'", input_val);
                                            match echo_server(input_val).await {
                                                Ok(data) => {
                                                    if request_id() == current_request_id {
                                                        println!("✅ Real-time echo success: '{}'", data);
                                                        response.set(data);
                                                        is_loading.set(false);
                                                    }
                                                }
                                                Err(e) => {
                                                    if request_id() == current_request_id {
                                                        println!("❌ Real-time echo error: {:?}", e);
                                                        response.set(format!("Error: {}", e));
                                                        is_loading.set(false);
                                                    }
                                                }
                                            }
                                        }
                                    });
                                } else {
                                    response.set(String::new());
                                    is_loading.set(false);
                                }
                            },
                        }
                        if is_loading() {
                            div { class: "bg-ctp-yellow/10 border border-ctp-yellow/40 rounded-md p-4 animate-pulse",
                                div { class: "flex items-center space-x-3",
                                    span { class: "text-lg", "⏳" }
                                    span { class: "text-ctp-yellow font-mono text-sm", "Server processing..." }
                                }
                            }
                        } else if !response().is_empty() {
                            div { class: "bg-ctp-green/10 border border-ctp-green/40 rounded-md p-4 animate-fade-in",
                                div { class: "flex items-center space-x-3",
                                    span { class: "text-lg", "✨" }
                                    span { class: "text-ctp-green font-mono text-sm", "{response}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
