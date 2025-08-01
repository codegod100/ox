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
        div {
            class: "min-h-screen bg-gradient-to-br from-ctp-base via-ctp-mantle to-ctp-crust relative overflow-hidden",

            // Animated background orbs
            div { class: "absolute inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-40 -left-40 w-80 h-80 bg-gradient-radial from-ctp-mauve/15 via-ctp-mauve/8 to-transparent rounded-full blur-3xl animate-float" }
                div { class: "absolute -bottom-40 -right-40 w-96 h-96 bg-gradient-radial from-ctp-blue/10 via-ctp-blue/5 to-transparent rounded-full blur-3xl animate-float" }
            }

            div {
                class: "relative z-10 pt-20 pb-16 px-6",
                div {
                    class: "container mx-auto max-w-4xl",

                    // Generated Content
                    if content_loading() {
                        div {
                            class: "flex flex-col items-center justify-center py-20 space-y-6",
                            div {
                                class: "relative",
                                div { class: "w-16 h-16 border-4 border-ctp-surface2 border-t-ctp-mauve rounded-full animate-spin" }
                                div { class: "absolute inset-0 w-16 h-16 border-4 border-transparent border-r-ctp-blue rounded-full animate-spin animate-reverse" }
                            }
                            div { class: "text-ctp-subtext0 text-xl font-light tracking-wide animate-pulse", "Loading content..." }
                        }
                    } else {
                        div {
                            class: "bg-gradient-to-br from-ctp-surface0/70 via-ctp-surface1/50 to-ctp-surface0/70 border border-ctp-surface2/50 backdrop-blur-2xl rounded-3xl p-12 shadow-glass mb-16 animate-fade-in",
                            h1 {
                                class: "text-5xl md:text-6xl font-extralight bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal bg-clip-text text-transparent mb-8 tracking-wide leading-tight",
                                "{blog_content().0}"
                            }
                            div {
                                class: "prose prose-lg prose-invert max-w-none text-ctp-text/90 leading-relaxed text-lg",
                                dangerous_inner_html: "{blog_content().1}"
                            }
                        }
                    }

                    // Cat Section
                    div {
                        class: "bg-gradient-to-br from-ctp-surface0/60 via-ctp-surface1/40 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-3xl p-10 shadow-glass animate-scale-in",

                        div { class: "text-center mb-8",
                            h3 {
                                class: "text-3xl font-light bg-gradient-to-r from-ctp-text via-ctp-subtext1 to-ctp-text bg-clip-text text-transparent mb-4 tracking-wide",
                                "Random Cat of the Day"
                            }
                            div { class: "w-24 h-0.5 bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal rounded-full mx-auto opacity-60" }
                        }

                        div { class: "flex justify-center mb-8",
                            button {
                                class: "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-mauve/80 to-ctp-blue/60 border border-ctp-mauve/60 backdrop-blur-xl hover:from-ctp-mauve hover:to-ctp-blue text-ctp-base hover:text-ctp-crust transition-all duration-300 rounded-2xl font-medium tracking-wide shadow-glow hover:shadow-glow-lg hover:scale-105",
                                onclick: move |_| async move {
                                    loading.set(true);
                                    if let Ok(url) = get_random_cat().await {
                                        cat_url.set(url);
                                    }
                                    loading.set(false);
                                },

                                div { class: "absolute inset-0 bg-shimmer-gradient opacity-0 group-hover:opacity-100 transition-opacity duration-500 group-hover:animate-shine" }

                                span { class: "relative z-10 flex items-center space-x-2",
                                    if loading() {
                                        span { class: "animate-spin text-lg", "🔄" }
                                        span { "Loading..." }
                                    } else {
                                        span { class: "text-lg", "🐱" }
                                        span { "Get Random Cat" }
                                    }
                                }
                            }
                        }

                        if !cat_url().is_empty() {
                            div {
                                class: "flex justify-center animate-fade-in",
                                div {
                                    class: "relative group",
                                    img {
                                        src: "{cat_url}",
                                        alt: "Random cat",
                                        class: "max-w-full max-h-96 rounded-2xl shadow-2xl cursor-pointer transition-all duration-500 group-hover:scale-105 group-hover:shadow-glow",
                                        onclick: move |_| {
                                            show_confetti.set(true);
                                            spawn(async move {
                                                gloo_timers::future::TimeoutFuture::new(2000).await;
                                                show_confetti.set(false);
                                            });
                                        },
                                    }
                                    div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/20 via-transparent to-ctp-blue/20 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none" }
                                }
                            }
                        }
                    }

                    // Confetti explosion
                    if show_confetti() {
                        div {
                            class: "fixed top-1/2 left-1/2 w-0 h-0 pointer-events-none z-50",
                            for i in 0..25 {
                                {
                                    let angle = (i as f32 * 14.4 + (i as f32 * 23.7).sin() * 45.0).to_radians();
                                    let speed = 200.0 + ((i as f32 * 17.3).sin() * 150.0);
                                    let dx = (angle.cos() * speed) as i32;
                                    let dy = (angle.sin() * speed) as i32;
                                    let rotation = ((i as f32 * 37.1).sin() * 360.0) as i32;
                                    let colors = ["bg-ctp-red", "bg-ctp-peach", "bg-ctp-yellow", "bg-ctp-green", "bg-ctp-blue", "bg-ctp-mauve"];
                                    rsx! {
                                        div {
                                            key: "{i}",
                                            class: format!("absolute w-3 h-3 rounded-full {} animate-ping", colors[i % 6]),
                                            style: format!("transform: translate({}px, {}px) rotate({}deg); animation-duration: 1.2s;", dx, dy, rotation),
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Navigation
                    div {
                        class: "flex justify-center items-center space-x-8 mt-16 pt-8 border-t border-ctp-surface2/30",
                        if id > 1 {
                            Link {
                                to: crate::types::Route::Blog { id: id - 1 },
                                class: "group flex items-center space-x-2 px-6 py-3 bg-gradient-to-r from-ctp-surface1/60 to-ctp-surface2/40 border border-ctp-surface2/50 backdrop-blur-xl hover:border-ctp-text/50 text-ctp-text hover:text-ctp-mauve transition-all duration-300 rounded-xl font-medium tracking-wide shadow-lg hover:shadow-glow hover:scale-105",
                                span { class: "text-lg group-hover:-translate-x-1 transition-transform duration-300", "←" }
                                span { "Previous" }
                            }
                        }

                        div { class: "w-px h-6 bg-gradient-to-b from-transparent via-ctp-surface2 to-transparent" }

                        Link {
                            to: crate::types::Route::Blog { id: id + 1 },
                            class: "group flex items-center space-x-2 px-6 py-3 bg-gradient-to-r from-ctp-surface1/60 to-ctp-surface2/40 border border-ctp-surface2/50 backdrop-blur-xl hover:border-ctp-text/50 text-ctp-text hover:text-ctp-mauve transition-all duration-300 rounded-xl font-medium tracking-wide shadow-lg hover:shadow-glow hover:scale-105",
                            span { "Next" }
                            span { class: "text-lg group-hover:translate-x-1 transition-transform duration-300", "→" }
                        }
                    }
                }
            }
        }
    }
}
