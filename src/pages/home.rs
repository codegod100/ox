use crate::server::echo_server;
use crate::types::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    println!("🏠 Home route matched - rendering Home component");
    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-ctp-base via-ctp-mantle to-ctp-crust relative overflow-hidden",

            // Animated background orbs
            div { class: "absolute inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-40 -left-40 w-80 h-80 bg-gradient-radial from-ctp-mauve/20 via-ctp-mauve/10 to-transparent rounded-full blur-3xl animate-float" }
                div { class: "absolute -bottom-40 -right-40 w-96 h-96 bg-gradient-radial from-ctp-blue/15 via-ctp-blue/8 to-transparent rounded-full blur-3xl animate-float" }
                div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-72 h-72 bg-gradient-radial from-ctp-teal/15 via-ctp-teal/8 to-transparent rounded-full blur-3xl animate-float" }
            }

            Hero {}
            Features {}
            Echo {}
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            class: "relative z-10 min-h-screen flex items-center justify-center px-6 pt-20",
            div {
                class: "text-center max-w-4xl mx-auto animate-fade-in",
                div {
                    class: "relative mb-8",
                    h1 {
                        class: "text-8xl md:text-9xl font-thin bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal bg-clip-text text-transparent mb-6 tracking-wider leading-none bg-[length:200%_100%] animate-shimmer",
                        "Ox"
                    }
                    div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/20 via-ctp-blue/20 to-ctp-teal/20 blur-3xl opacity-60 -z-10" }
                }

                p {
                    class: "text-2xl md:text-3xl text-ctp-subtext0/90 font-extralight mb-12 leading-relaxed tracking-wide max-w-3xl mx-auto",
                    "Modern Rust web applications with Dioxus"
                }

                div {
                    class: "flex justify-center animate-slide-in",
                    Link {
                        to: Route::Blog { id: 1 },
                        class: "group relative overflow-hidden px-12 py-5 bg-gradient-to-r from-ctp-mauve/80 to-ctp-blue/60 border border-ctp-mauve/60 backdrop-blur-xl hover:from-ctp-mauve hover:to-ctp-blue text-ctp-base hover:text-ctp-crust transition-all duration-500 rounded-2xl font-medium tracking-wide shadow-glow hover:shadow-glow-lg hover:scale-110 text-xl",

                        div { class: "absolute inset-0 bg-shimmer-gradient opacity-0 group-hover:opacity-100 transition-opacity duration-700 group-hover:animate-shine" }

                        span { class: "relative z-10 flex items-center space-x-3",
                            span { "Get Started" }
                            span { class: "text-2xl group-hover:translate-x-1 transition-transform duration-300", "→" }
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
            class: "relative z-10 py-20 px-6",
            div {
                class: "container mx-auto max-w-6xl",
                div {
                    class: "text-center mb-16",
                    h2 {
                        class: "text-5xl font-extralight bg-gradient-to-r from-ctp-text via-ctp-subtext1 to-ctp-text bg-clip-text text-transparent mb-6 tracking-wider",
                        "Built for Excellence"
                    }
                    div { class: "w-24 h-0.5 bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal rounded-full mx-auto opacity-70" }
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-8",

                    div {
                        class: "group bg-gradient-to-br from-ctp-surface0/60 via-ctp-surface1/40 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-3xl p-8 shadow-glass hover:shadow-glow transition-all duration-500 hover:scale-105 animate-scale-in",
                        div { class: "text-center",
                            div { class: "text-5xl mb-6 group-hover:scale-110 transition-transform duration-300", "🛡️" }
                            h3 { class: "text-2xl font-medium text-ctp-text mb-4 tracking-wide group-hover:text-ctp-mauve transition-colors duration-300", "Type Safe" }
                            p { class: "text-ctp-subtext0/80 font-light leading-relaxed", "Built with Rust's type system for reliability" }
                        }
                    }

                    div {
                        class: "group bg-gradient-to-br from-ctp-surface0/60 via-ctp-surface1/40 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-3xl p-8 shadow-glass hover:shadow-glow transition-all duration-500 hover:scale-105 animate-scale-in",
                        style: "animation-delay: 200ms",
                        div { class: "text-center",
                            div { class: "text-5xl mb-6 group-hover:scale-110 transition-transform duration-300", "⚡" }
                            h3 { class: "text-2xl font-medium text-ctp-text mb-4 tracking-wide group-hover:text-ctp-blue transition-colors duration-300", "Fast" }
                            p { class: "text-ctp-subtext0/80 font-light leading-relaxed", "Optimized for performance and low latency" }
                        }
                    }

                    div {
                        class: "group bg-gradient-to-br from-ctp-surface0/60 via-ctp-surface1/40 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-3xl p-8 shadow-glass hover:shadow-glow transition-all duration-500 hover:scale-105 animate-scale-in",
                        style: "animation-delay: 400ms",
                        div { class: "text-center",
                            div { class: "text-5xl mb-6 group-hover:scale-110 transition-transform duration-300", "🔗" }
                            h3 { class: "text-2xl font-medium text-ctp-text mb-4 tracking-wide group-hover:text-ctp-teal transition-colors duration-300", "Full Stack" }
                            p { class: "text-ctp-subtext0/80 font-light leading-relaxed", "Server and client code in one codebase" }
                        }
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
            class: "relative z-10 py-20 px-6",
            div {
                class: "container mx-auto max-w-2xl",
                div {
                    class: "bg-gradient-to-br from-ctp-surface0/70 via-ctp-surface1/50 to-ctp-surface0/70 border border-ctp-surface2/50 backdrop-blur-2xl rounded-3xl p-12 shadow-glass animate-scale-in",

                    div { class: "text-center mb-8",
                        h3 {
                            class: "text-3xl font-light bg-gradient-to-r from-ctp-text via-ctp-subtext1 to-ctp-text bg-clip-text text-transparent mb-4 tracking-wide",
                            "Server Echo Demo"
                        }
                        div { class: "w-16 h-0.5 bg-gradient-to-r from-ctp-mauve to-ctp-blue rounded-full mx-auto opacity-60" }
                    }

                    div { class: "space-y-6",
                        div { class: "relative group",
                            input {
                                class: "w-full px-6 py-4 bg-gradient-to-br from-ctp-base/80 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl text-ctp-text text-lg placeholder-ctp-subtext0/60 transition-all duration-500 focus:outline-none focus:border-ctp-mauve/60 focus:shadow-glow focus:bg-ctp-base/90 hover:border-ctp-text/40",
                                placeholder: "Type something magical...",
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
                            div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/10 via-ctp-blue/10 to-ctp-teal/10 rounded-2xl opacity-0 group-focus-within:opacity-100 transition-opacity duration-500 pointer-events-none blur-sm" }
                        }

                        if !response().is_empty() {
                            div {
                                class: "relative overflow-hidden bg-gradient-to-br from-ctp-surface1/60 to-ctp-surface2/40 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl p-6 shadow-inner-glow animate-fade-in",
                                div { class: "absolute inset-0 bg-gradient-to-r from-ctp-green/5 to-ctp-blue/5" }
                                div { class: "relative z-10 flex items-center space-x-3",
                                    span { class: "text-2xl animate-pulse", "✨" }
                                    span { class: "text-ctp-text font-mono text-lg", "{response}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
