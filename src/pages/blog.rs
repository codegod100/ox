use crate::server::{generate_blog_content, get_random_cat};
use dioxus::prelude::*;

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
    use_effect(move || {
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
    });

    rsx! {
        div { class: "min-h-screen bg-ctp-base",
            div { class: "pt-32 pb-16 px-6",
                div { class: "container mx-auto max-w-3xl",

                    // Generated Content
                    if content_loading() {
                        div { class: "flex flex-col items-center justify-center py-20 space-y-4",
                            div { class: "w-8 h-8 border-2 border-ctp-surface2 border-t-ctp-mauve rounded-full animate-spin" }
                            div { class: "text-ctp-subtext0 text-lg", "Loading content..." }
                        }
                    } else {
                        article { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-lg p-8 mb-12 animate-fade-in",
                            h1 { class: "text-3xl md:text-4xl font-light text-ctp-mauve mb-6",
                                "{blog_content().0}"
                            }
                            div {
                                class: "prose prose-lg prose-invert max-w-none text-ctp-text leading-relaxed",
                                dangerous_inner_html: "{blog_content().1}",
                            }
                        }
                    }

                    // Cat Section
                    div { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-lg p-8",

                        h3 { class: "text-xl font-medium text-ctp-text text-center mb-6",
                            "Random Cat of the Day"
                        }

                        div { class: "flex justify-center mb-6",
                            button {
                                class: "px-6 py-3 border-2 border-ctp-mauve text-ctp-mauve hover:bg-ctp-mauve hover:text-ctp-base bg-transparent transition-colors rounded-md font-medium flex items-center space-x-2",
                                onclick: move |_| async move {
                                    loading.set(true);
                                    if let Ok(url) = get_random_cat().await {
                                        cat_url.set(url);
                                    }
                                    loading.set(false);
                                },

                                if loading() {
                                    span { class: "animate-spin text-lg", "🔄" }
                                    span { "Loading..." }
                                } else {
                                    span { class: "text-lg", "🐱" }
                                    span { "Get Random Cat" }
                                }
                            }
                        }

                        if !cat_url().is_empty() {
                            div { class: "flex justify-center animate-fade-in",
                                img {
                                    src: "{cat_url}",
                                    alt: "Random cat",
                                    class: "max-w-full max-h-80 rounded-lg shadow-lg cursor-pointer hover:scale-105 transition-transform duration-300",
                                    onclick: move |_| {
                                        show_confetti.set(true);
                                        spawn(async move {
                                            gloo_timers::future::TimeoutFuture::new(2000).await;
                                            show_confetti.set(false);
                                        });
                                    },
                                }
                            }
                        }
                    }

                    // Confetti explosion
                    if show_confetti() {
                        div { class: "fixed top-1/2 left-1/2 w-0 h-0 pointer-events-none z-50",
                            for i in 0..30 {
                                {
                                    let angle = (i as f32 * 12.0 + (i as f32 * 23.7).sin() * 60.0).to_radians();
                                    let speed = 150.0 + ((i as f32 * 17.3).sin() * 200.0);
                                    let dx = (angle.cos() * speed) as i32;
                                    let dy = (angle.sin() * speed) as i32;
                                    let rotation = ((i as f32 * 37.1).sin() * 720.0) as i32;
                                    rsx! {
                                        div {
                                            key: "{i}",
                                            class: format!("confetti-burst confetti-{}", i % 6),
                                            style: format!("--dx: {}; --dy: {}; --rotation: {}deg;", dx, dy, rotation),
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Navigation
                    div { class: "flex justify-center items-center space-x-6 mt-12 pt-6 border-t border-ctp-surface1/50",
                        if id > 1 {
                            Link {
                                to: crate::types::Route::Blog {
                                    id: id - 1,
                                },
                                class: "flex items-center space-x-2 px-4 py-2 bg-ctp-surface1 text-ctp-text hover:text-ctp-mauve hover:bg-ctp-surface2 transition-colors rounded-md",
                                span { "←" }
                                span { "Previous" }
                            }
                        }

                        span { class: "text-ctp-subtext0", "•" }

                        Link {
                            to: crate::types::Route::Blog {
                                id: id + 1,
                            },
                            class: "flex items-center space-x-2 px-4 py-2 bg-ctp-surface1 text-ctp-text hover:text-ctp-mauve hover:bg-ctp-surface2 transition-colors rounded-md",
                            span { "Next" }
                            span { "→" }
                        }
                    }
                }
            }
        }
    }
}
