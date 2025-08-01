use crate::types::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    println!("🧭 Navbar rendering");
    rsx! {
        div {
            class: "fixed top-0 left-0 right-0 z-50 bg-ctp-base/80 backdrop-blur-sm border-b border-ctp-surface1",
            nav {
                class: "max-w-6xl mx-auto px-6 py-4",
                div {
                    class: "flex items-center justify-between",
                    Link {
                        to: Route::Home {},
                        class: "text-2xl font-light text-ctp-mauve hover:text-ctp-blue transition-colors",
                        "Ox"
                    }
                    div {
                        class: "flex items-center space-x-8",
                        NavLink { to: Route::Home {}, text: "Home" }
                        NavLink { to: Route::Blog { id: 1 }, text: "Blog" }
                        NavLink { to: Route::Tools {}, text: "Tools" }
                    }
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
            to,
            class: "text-ctp-text hover:text-ctp-mauve transition-colors font-medium relative group",
            "{text}"
        }
    }
}
