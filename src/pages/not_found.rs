use dioxus::prelude::*;

/// 404 Not Found page - logs unmatched routes
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    println!("❌ 404 - Route not found: '{}'", path);
    println!("❌ Segments: {:?}", segments);

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-ctp-base via-ctp-mantle to-ctp-crust relative overflow-hidden flex items-center justify-center",

            // Animated background orbs
            div { class: "absolute inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-40 -left-40 w-80 h-80 bg-gradient-radial from-ctp-red/15 via-ctp-red/8 to-transparent rounded-full blur-3xl animate-float" }
                div { class: "absolute -bottom-40 -right-40 w-96 h-96 bg-gradient-radial from-ctp-mauve/10 via-ctp-mauve/5 to-transparent rounded-full blur-3xl animate-float" }
            }

            div {
                class: "relative z-10 text-center px-6 max-w-4xl mx-auto",
                div {
                    class: "bg-gradient-to-br from-ctp-surface0/70 via-ctp-surface1/50 to-ctp-surface0/70 border border-ctp-surface2/50 backdrop-blur-2xl rounded-3xl p-12 shadow-glass animate-scale-in",

                    // 404 Icon
                    div { class: "text-8xl mb-8 animate-bounce", "🚫" }

                    h1 {
                        class: "text-6xl md:text-7xl font-extralight bg-gradient-to-r from-ctp-red via-ctp-mauve to-ctp-red bg-clip-text text-transparent mb-6 tracking-wider",
                        "404"
                    }

                    h2 {
                        class: "text-3xl font-light text-ctp-text mb-8 tracking-wide",
                        "Page Not Found"
                    }

                    p {
                        class: "text-lg text-ctp-subtext0/80 font-light mb-8 leading-relaxed",
                        "The requested path '", span { class: "font-mono text-ctp-red", "{path}" }, "' was not found."
                    }

                    div {
                        class: "bg-gradient-to-br from-ctp-surface1/60 to-ctp-surface2/40 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl p-8 mb-10",
                        h3 {
                            class: "text-xl font-medium text-ctp-text mb-4 tracking-wide",
                            "Available routes:"
                        }
                        div {
                            class: "space-y-3 text-left",
                            div { class: "flex items-center space-x-3",
                                span { class: "text-ctp-green text-lg", "✓" }
                                span { class: "font-mono text-ctp-text bg-ctp-base/50 px-3 py-1 rounded-lg", "/" }
                                span { class: "text-ctp-subtext0/80 font-light", "- Home page" }
                            }
                            div { class: "flex items-center space-x-3",
                                span { class: "text-ctp-green text-lg", "✓" }
                                span { class: "font-mono text-ctp-text bg-ctp-base/50 px-3 py-1 rounded-lg", "/blog/[id]" }
                                span { class: "text-ctp-subtext0/80 font-light", "- Blog posts" }
                            }
                            div { class: "flex items-center space-x-3",
                                span { class: "text-ctp-green text-lg", "✓" }
                                span { class: "font-mono text-ctp-text bg-ctp-base/50 px-3 py-1 rounded-lg", "/tools" }
                                span { class: "text-ctp-subtext0/80 font-light", "- Developer tools" }
                            }
                        }
                    }

                    Link {
                        to: crate::types::Route::Home {},
                        class: "group relative overflow-hidden inline-flex items-center space-x-3 px-8 py-4 bg-gradient-to-r from-ctp-mauve/80 to-ctp-blue/60 border border-ctp-mauve/60 backdrop-blur-xl hover:from-ctp-mauve hover:to-ctp-blue text-ctp-base hover:text-ctp-crust transition-all duration-300 rounded-2xl font-medium tracking-wide shadow-glow hover:shadow-glow-lg hover:scale-105",

                        div { class: "absolute inset-0 bg-shimmer-gradient opacity-0 group-hover:opacity-100 transition-opacity duration-500 group-hover:animate-shine" }

                        span { class: "relative z-10 flex items-center space-x-2",
                            span { class: "text-lg group-hover:-translate-x-1 transition-transform duration-300", "←" }
                            span { "Back to Home" }
                        }
                    }
                }
            }
        }
    }
}
