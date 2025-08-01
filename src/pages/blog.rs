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
        div { id: "blog",

            // Generated Content
            if content_loading() {
                div { class: "loading", "Loading content..." }
            } else {
                div {
                    h1 { "{blog_content().0}" }
                    div { class: "blog-content", "{blog_content().1}" }
                }
            }

            // Cat Section
            div { class: "cat-section",
                h3 { "Random Cat of the Day" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| async move {
                        loading.set(true);
                        if let Ok(url) = get_random_cat().await {
                            cat_url.set(url);
                        }
                        loading.set(false);
                    },
                    if loading() {
                        "Loading..."
                    } else {
                        "Get Random Cat"
                    }
                }

                if !cat_url().is_empty() {
                    div { class: "cat-display",
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
                            },
                        }
                    }
                }
            }

            // Confetti explosion
            if show_confetti() {
                div { class: "confetti-container",
                    for i in 0..25 {
                        {
                            let angle = (i as f32 * 14.4 + (i as f32 * 23.7).sin() * 45.0).to_radians();
                            let speed = 200.0 + ((i as f32 * 17.3).sin() * 150.0);
                            let dx = (angle.cos() * speed) as i32;
                            let dy = (angle.sin() * speed) as i32;
                            let rotation = ((i as f32 * 37.1).sin() * 360.0) as i32;
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
            div { class: "blog-nav",
                if id > 1 {
                    Link {
                        to: crate::types::Route::Blog {
                            id: id - 1,
                        },
                        "← Previous"
                    }
                }
                span { "|" }
                Link {
                    to: crate::types::Route::Blog {
                        id: id + 1,
                    },
                    "Next →"
                }
            }
        }
    }
}
