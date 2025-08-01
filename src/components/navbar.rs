use dioxus::prelude::*;
use crate::types::Route;

#[component]
pub fn Navbar() -> Element {
    println!("🧭 Navbar rendering");
    rsx! {  
        div {
            id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
            Link { to: Route::Tools {}, "Tools" }
        }
        Outlet::<Route> {}
    }
}