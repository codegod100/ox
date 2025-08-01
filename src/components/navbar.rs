use crate::types::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    println!("🧭 Navbar rendering");
    rsx! {
        div {
            class: "fixed top-0 left-0 right-0 z-50 bg-gradient-to-r from-ctp-base/80 via-ctp-mantle/80 to-ctp-base/80 backdrop-blur-2xl border-b border-ctp-surface1/50 shadow-glass",
            nav {
                class: "container mx-auto px-6 py-4",
                div {
                    class: "flex justify-center items-center space-x-8",
                    NavLink { to: Route::Home {}, text: "Home" }
                    NavLink { to: Route::Blog { id: 1 }, text: "Blog" }
                    NavLink { to: Route::Tools {}, text: "Tools" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn NavLink(to: Route, text: String) -> Element {
    rsx! {
        Link {
            to: to,
            class: "group relative px-6 py-3 text-ctp-subtext0 hover:text-ctp-text transition-all duration-300 font-medium tracking-wide",

            // Hover background effect
            div {
                class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/10 via-ctp-blue/10 to-ctp-teal/10 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 scale-95 group-hover:scale-100"
            }

            // Bottom border effect
            div {
                class: "absolute bottom-0 left-1/2 w-0 h-0.5 bg-gradient-to-r from-ctp-mauve to-ctp-blue rounded-full group-hover:w-full group-hover:left-0 transition-all duration-300"
            }

            span {
                class: "relative z-10",
                "{text}"
            }
        }
    }
}
