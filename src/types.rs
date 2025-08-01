use crate::components::Navbar;
use crate::pages::{Blog, Home, NotFound, Tools};
use dioxus::prelude::*;
use std::fmt;

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
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/tools")]
    Tools {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
