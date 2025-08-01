# Agent Guidelines for Ox

## Build/Test Commands
- Don't use any build commands automatically unless specifically told to do so.
- don't use 'cargoo check' or any other cargo commands

## Code Style
- **Language**: Rust with Dioxus framework for fullstack web apps
- **Imports**: Use `use crate::` for internal modules, group std/external/internal imports
- **Components**: Use `#[component]` macro, PascalCase names, return `Element`
- **Props**: Use `Signal<T>` for reactive state, destructure props in function signature
- **Naming**: snake_case for variables/functions, PascalCase for components/types
- **Error handling**: Use `Result<T, E>` and `match` for explicit error handling
- **Formatting**: Use `cargo fmt` with default rustfmt settings
- **Comments**: Minimal comments, prefer self-documenting code

## RSX Macro Caveats
- **No comments in RSX**: Never put `//` comments inside `rsx!` blocks - they cause parse errors
- **Curly braces in strings**: Avoid `"{ }"` in text content - RSX interprets `{}` as syntax
- **Nested rsx! calls**: Don't nest `rsx!` macros in component props - inline the content instead
- **String literals**: Use simple strings without special characters that could confuse the parser
- **Component structure**: Keep RSX structure flat when possible to avoid macro parsing issues

## Tailwind CSS Build Process
- **CSS Source**: Edit `input.css` for custom styles and Tailwind imports
- **Build Command**: `bunx tailwindcss -i input.css -o assets/tailwind.css --minify`
- **Output Location**: Always build CSS to `assets/tailwind.css` (not root directory)
- **Catppuccin Colors**: Use `ctp-*` classes (e.g., `bg-ctp-base`, `text-ctp-mauve`)
- **Development**: CSS is auto-built during Docker build process

## Architecture
- **Structure**: `src/` contains `main.rs`, `lib.rs`, `components/`, `pages/`, `server.rs`, `types.rs`
- **Routing**: Uses Dioxus router with `#[derive(Routable)]` enum in `types.rs`
- **Styling**: Tailwind CSS v4 with Catppuccin color scheme (`ctp-*` classes)
- **Assets**: Static files in `assets/` directory, use `asset!()` macro for references
