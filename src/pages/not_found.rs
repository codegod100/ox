use dioxus::prelude::*;

/// 404 Not Found page - logs unmatched routes
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
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
                li { "/tools" }
            }
            Link {
                to: crate::types::Route::Home {},
                "← Back to Home"
            }
        }
    }
}