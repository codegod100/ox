use dioxus::prelude::*;
use std::fmt;

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
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Add comprehensive startup logging
    println!("🚀 Starting Ox server...");
    println!("📍 Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
    println!("🌐 PORT: {}", std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()));
    println!("🔗 IP: {}", std::env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string()));
    
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
    
    dioxus::launch(App);
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
    println!("📝 Blog route matched - rendering Blog component with id: {}", id);
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
        }

        Outlet::<Route> {}
    }
}

/// Simple demo of server functions
#[component]
fn Echo() -> Element {
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
                        oninput: move |event| async move {
                            let data = echo_server(event.value()).await.unwrap_or_else(|_| "Error".to_string());
                            response.set(data);
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
    println!("📡 Server function called: echo_server with input: '{}'", input);
    let response = format!("You said: {}", input);
    println!("📡 echo_server responding: '{}'", response);
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
    
    let url = json[0]["url"]
        .as_str()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("No cat URL found".to_string()))?;
    
    Ok(url.to_string())
}

#[server(GenerateBlogContent)]
async fn generate_blog_content(blog_id: i32) -> Result<(String, String), ServerFnError> {
    println!("📡 Server function called: generate_blog_content with blog_id: {}", blog_id);
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

