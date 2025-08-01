use dioxus::prelude::*;

/// 404 Not Found page - logs unmatched routes
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    println!("❌ 404 - Route not found: '{}'", path);
    println!("❌ Segments: {:?}", segments);

    rsx! {
        div { class: "min-h-screen bg-ctp-base flex items-center justify-center",
            div { class: "text-center px-6 max-w-lg mx-auto",
                div { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-lg p-8",

                    // 404 Icon
                    div { class: "text-6xl mb-6", "🚫" }

                    h1 { class: "text-5xl font-light text-ctp-red mb-4", "404" }

                    h2 { class: "text-2xl font-medium text-ctp-text mb-6", "Page Not Found" }

                    p { class: "text-ctp-subtext0/70 mb-8 leading-relaxed",
                        "The requested path '"
                        span { class: "font-mono text-ctp-red bg-ctp-surface1 px-2 py-1 rounded",
                            "{path}"
                        }
                        "' was not found."
                    }

                    div { class: "bg-ctp-surface1/50 rounded-md p-6 mb-8 text-left",
                        h3 { class: "text-lg font-medium text-ctp-text mb-4", "Available routes:" }
                        div { class: "space-y-2",
                            div { class: "flex items-center space-x-3",
                                span { class: "text-ctp-green", "✓" }
                                span { class: "font-mono text-sm text-ctp-text bg-ctp-base px-2 py-1 rounded",
                                    "/"
                                }
                                span { class: "text-sm text-ctp-subtext0/70", "- Home page" }
                            }
                            div { class: "flex items-center space-x-3",
                                span { class: "text-ctp-green", "✓" }
                                span { class: "font-mono text-sm text-ctp-text bg-ctp-base px-2 py-1 rounded",
                                    "/blog/[id]"
                                }
                                span { class: "text-sm text-ctp-subtext0/70", "- Blog posts" }
                            }
                            div { class: "flex items-center space-x-3",
                                span { class: "text-ctp-green", "✓" }
                                span { class: "font-mono text-sm text-ctp-text bg-ctp-base px-2 py-1 rounded",
                                    "/tools"
                                }
                                span { class: "text-sm text-ctp-subtext0/70", "- Developer tools" }
                            }
                        }
                    }

                    Link {
                        to: crate::types::Route::Home {},
                        class: "inline-flex items-center space-x-2 px-6 py-3 bg-ctp-mauve text-ctp-base hover:bg-ctp-blue transition-colors rounded-md font-medium",
                        span { "←" }
                        span { "Back to Home" }
                    }
                }
            }
        }
    }
}
