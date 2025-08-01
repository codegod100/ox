use crate::server::echo_server;
use crate::types::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    println!("🏠 Home route matched - rendering Home component");
    rsx! {
        Hero {}
        Features {}
        Echo {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            class: "hero-section",
            div {
                class: "container",
                div {
                    class: "hero-container",
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
}

#[component]
pub fn Features() -> Element {
    rsx! {
        section {
            class: "features-section",
            div {
                class: "container",
                div {
                    class: "features-list",
                    div {
                        class: "feature-item",
                        h3 { "Type Safe" }
                        p { "Built with Rust's type system for reliability" }
                    }
                    div {
                        class: "feature-item",
                        h3 { "Fast" }
                        p { "Optimized for performance and low latency" }
                    }
                    div {
                        class: "feature-item",
                        h3 { "Full Stack" }
                        p { "Server and client code in one codebase" }
                    }
                }
            }
        }
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
