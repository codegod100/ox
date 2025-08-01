use crate::components::tools::*;
use dioxus::prelude::*;

// ============================================================================
// DATA STRUCTURES & CONFIGURATION
// ============================================================================

#[derive(Clone, PartialEq)]
struct TabInfo {
    id: &'static str,
    icon: &'static str,
    title: &'static str,
    subtitle: &'static str,
}

const TABS: [TabInfo; 4] = [
    TabInfo {
        id: "json",
        icon: "{ }",
        title: "JSON",
        subtitle: "Format & Validate",
    },
    TabInfo {
        id: "text",
        icon: "Aa",
        title: "Text",
        subtitle: "Transform & Count",
    },
    TabInfo {
        id: "qr",
        icon: "⚡",
        title: "QR Code",
        subtitle: "Generate Codes",
    },
    TabInfo {
        id: "base64",
        icon: "🔒",
        title: "Base64",
        subtitle: "Encode & Decode",
    },
];

// ============================================================================
// BUSINESS LOGIC
// ============================================================================

#[derive(Clone, PartialEq)]
struct ToolsState {
    active_tab: Signal<String>,
    is_initialized: Signal<bool>,
    json_input: Signal<String>,
    json_output: Signal<String>,
    json_error: Signal<String>,
    text_input: Signal<String>,
    word_count: Signal<i32>,
    char_count: Signal<i32>,
    qr_text: Signal<String>,
    qr_url: Signal<String>,
    base64_input: Signal<String>,
    base64_output: Signal<String>,
}

impl ToolsState {
    fn new() -> Self {
        Self {
            active_tab: use_signal(|| "json".to_string()),
            is_initialized: use_signal(|| false),
            json_input: use_signal(|| String::new()),
            json_output: use_signal(|| String::new()),
            json_error: use_signal(|| String::new()),
            text_input: use_signal(|| String::new()),
            word_count: use_signal(|| 0),
            char_count: use_signal(|| 0),
            qr_text: use_signal(|| String::new()),
            qr_url: use_signal(|| String::new()),
            base64_input: use_signal(|| String::new()),
            base64_output: use_signal(|| String::new()),
        }
    }

    fn initialize(&mut self) {
        let mut is_initialized = self.is_initialized;
        use_effect(move || {
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(100).await;
                is_initialized.set(true);
            });
        });
    }

    fn get_active_tab(&self) -> String {
        (self.active_tab)()
    }

    fn is_initialized(&self) -> bool {
        (self.is_initialized)()
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn get_tool_title(tab_id: &str) -> &'static str {
    match tab_id {
        "json" => "JSON Formatter & Validator",
        "text" => "Text Utilities",
        "qr" => "QR Code Generator",
        "base64" => "Base64 Encoder/Decoder",
        _ => "Unknown Tool",
    }
}

fn get_tool_description(tab_id: &str) -> &'static str {
    match tab_id {
        "json" => "Format, validate, and prettify your JSON data with precision",
        "text" => "Transform text case and analyze character & word counts",
        "qr" => "Generate sleek QR codes for text, URLs, and any data",
        "base64" => "Encode and decode Base64 strings with modern efficiency",
        _ => "Tool description not available",
    }
}

// ============================================================================
// VIEW COMPONENTS
// ============================================================================

#[component]
pub fn Tools() -> Element {
    let mut state = ToolsState::new();
    state.initialize();

    if !state.is_initialized() {
        return rsx! { LoadingView {} };
    }

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-ctp-base via-ctp-mantle to-ctp-crust relative overflow-hidden",

            // Animated background orbs
            div { class: "absolute inset-0 overflow-hidden pointer-events-none",
                div { class: "absolute -top-40 -left-40 w-80 h-80 bg-gradient-radial from-ctp-mauve/20 via-ctp-mauve/10 to-transparent rounded-full blur-3xl animate-float" }
                div { class: "absolute -bottom-40 -right-40 w-96 h-96 bg-gradient-radial from-ctp-blue/15 via-ctp-blue/8 to-transparent rounded-full blur-3xl animate-float" }
                div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-72 h-72 bg-gradient-radial from-ctp-teal/15 via-ctp-teal/8 to-transparent rounded-full blur-3xl animate-float" }
            }

            PageHeader {}
            TabSelector { state: state.clone() }
            ToolContent { state: state }
            PageFooter {}
        }
    }
}

#[component]
fn LoadingView() -> Element {
    rsx! {
        div {
            class: "min-h-screen pt-20 pb-16 flex items-center justify-center relative z-10",
            div {
                class: "flex flex-col items-center space-y-6 animate-fade-in",
                div {
                    class: "relative",
                    div { class: "w-16 h-16 border-4 border-ctp-surface2 border-t-ctp-mauve rounded-full animate-spin" }
                    div { class: "absolute inset-0 w-16 h-16 border-4 border-transparent border-r-ctp-blue rounded-full animate-spin animate-reverse" }
                }
                div {
                    class: "text-ctp-subtext0 text-xl font-light tracking-wide animate-pulse",
                    "Loading tools..."
                }
                div { class: "w-32 h-1 bg-gradient-to-r from-transparent via-ctp-mauve to-transparent rounded-full animate-pulse" }
            }
        }
    }
}

#[component]
fn PageHeader() -> Element {
    rsx! {
        div {
            class: "pt-20 pb-16 relative z-10",
            div {
                class: "container mx-auto px-6 text-center",
                div {
                    class: "relative inline-block animate-slide-in",
                    h1 {
                        class: "text-7xl md:text-8xl font-thin bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal bg-clip-text text-transparent mb-8 tracking-wider leading-none bg-[length:200%_100%] animate-shimmer",
                        "Developer Tools"
                    }
                    div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/20 via-ctp-blue/20 to-ctp-teal/20 blur-3xl opacity-60 -z-10" }
                }
                p {
                    class: "text-xl md:text-2xl text-ctp-subtext0/90 font-extralight max-w-3xl mx-auto leading-relaxed tracking-wide animate-fade-in",
                    "Elegant utilities crafted for the modern developer experience"
                }
                div { class: "mt-8 w-24 h-0.5 bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal rounded-full mx-auto opacity-60" }
            }
        }
    }
}

#[component]
fn TabSelector(state: ToolsState) -> Element {
    let active_tab = state.get_active_tab();
    let mut active_tab_signal = state.active_tab;

    rsx! {
        div {
            class: "flex justify-center px-6 mb-32 relative z-10",
            div {
                class: "grid grid-cols-2 lg:grid-cols-4 gap-8 max-w-6xl animate-scale-in",
                for tab in TABS {
                    TabButton {
                        tab: tab.clone(),
                        is_active: active_tab == tab.id,
                        on_click: move |tab_id| active_tab_signal.set(tab_id)
                    }
                }
            }
        }
    }
}

#[component]
fn TabButton(tab: TabInfo, is_active: bool, on_click: EventHandler<String>) -> Element {
    let tab_id = tab.id.to_string();

    let button_classes = if is_active {
        "group relative overflow-hidden bg-gradient-to-br from-ctp-surface1/80 to-ctp-surface2/60 border border-ctp-mauve/60 backdrop-blur-xl rounded-3xl p-8 transition-all duration-700 shadow-glow-lg scale-105 transform"
    } else {
        "group relative overflow-hidden bg-gradient-to-br from-ctp-surface0/40 to-ctp-surface1/30 border border-ctp-surface2/40 backdrop-blur-xl rounded-3xl p-8 transition-all duration-700 hover:border-ctp-text/50 hover:shadow-glow hover:scale-105 hover:-translate-y-3 cursor-pointer"
    };

    rsx! {
        button {
            key: "{tab.id}",
            class: "{button_classes} min-w-[280px] min-h-[200px]",
            onclick: move |_| on_click.call(tab_id.clone()),

            // Animated gradient overlay
            div {
                class: if is_active {
                    "absolute inset-0 bg-gradient-to-br from-ctp-mauve/20 via-transparent to-ctp-blue/20 opacity-100 transition-opacity duration-500"
                } else {
                    "absolute inset-0 bg-gradient-to-br from-ctp-mauve/10 via-transparent to-ctp-blue/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"
                }
            }

            // Shine effect
            div {
                class: "absolute inset-0 bg-shimmer-gradient opacity-0 group-hover:opacity-100 transition-opacity duration-700 group-hover:animate-shine"
            }

            // Border glow effect
            div {
                class: if is_active {
                    "absolute inset-0 rounded-3xl bg-gradient-to-r from-ctp-mauve/30 via-ctp-blue/30 to-ctp-teal/30 blur-sm opacity-100"
                } else {
                    "absolute inset-0 rounded-3xl bg-gradient-to-r from-ctp-text/20 via-ctp-subtext1/20 to-ctp-text/20 blur-sm opacity-0 group-hover:opacity-100 transition-opacity duration-500"
                }
            }

            TabButtonContent { tab: tab }
        }
    }
}

#[component]
fn TabButtonContent(tab: TabInfo) -> Element {
    rsx! {
        div {
            class: "text-center relative z-10",
            div {
                class: "text-6xl mb-6 group-hover:scale-110 transition-transform duration-500 drop-shadow-lg",
                "{tab.icon}"
            }
            h3 {
                class: "font-medium text-ctp-text mb-3 text-2xl tracking-wider group-hover:text-ctp-mauve transition-colors duration-300",
                "{tab.title}"
            }
            p {
                class: "text-sm text-ctp-subtext0/80 font-light tracking-wide group-hover:text-ctp-subtext1 transition-colors duration-300",
                "{tab.subtitle}"
            }
        }
    }
}

#[component]
fn ToolContent(state: ToolsState) -> Element {
    let active_tab = state.get_active_tab();

    rsx! {
        div {
            class: "container mx-auto px-6 pb-20 relative z-10",
            div {
                class: "max-w-7xl mx-auto",
                div {
                    class: "relative overflow-hidden bg-gradient-to-br from-ctp-surface0/70 via-ctp-surface1/50 to-ctp-surface0/70 border border-ctp-surface2/50 backdrop-blur-4xl rounded-[2rem] p-12 shadow-glass animate-scale-in",

                    // Animated border gradient
                    div {
                        class: "absolute inset-0 rounded-[2rem] bg-gradient-to-r from-ctp-mauve/20 via-ctp-blue/20 to-ctp-teal/20 opacity-0 hover:opacity-100 transition-opacity duration-700 -z-10"
                    }

                    // Inner glow
                    div {
                        class: "absolute inset-4 rounded-3xl bg-gradient-to-br from-ctp-text/5 via-transparent to-ctp-text/5 pointer-events-none"
                    }

                    // Floating particles effect
                    div {
                        class: "absolute inset-0 overflow-hidden pointer-events-none",
                        div { class: "absolute top-10 left-10 w-2 h-2 bg-ctp-mauve/30 rounded-full animate-float" }
                        div { class: "absolute top-20 right-20 w-1 h-1 bg-ctp-blue/40 rounded-full animate-float" }
                        div { class: "absolute bottom-16 left-16 w-1.5 h-1.5 bg-ctp-teal/20 rounded-full animate-float" }
                    }

                    ToolHeader { tab_id: active_tab.clone() }
                    ToolRenderer { state: state, active_tab: active_tab }
                }
            }
        }
    }
}

#[component]
fn ToolHeader(tab_id: String) -> Element {
    let title = get_tool_title(&tab_id);
    let description = get_tool_description(&tab_id);

    rsx! {
        div {
            class: "space-y-8 mb-16",
            div {
                class: "text-center relative",

                // Background glow
                div {
                    class: "absolute inset-0 bg-gradient-to-r from-transparent via-ctp-text/5 to-transparent rounded-3xl blur-xl"
                }

                h2 {
                    class: "text-4xl md:text-5xl font-extralight bg-gradient-to-r from-ctp-text via-ctp-subtext1 to-ctp-text bg-clip-text text-transparent mb-6 tracking-wider relative z-10 leading-tight",
                    "{title}"
                }

                p {
                    class: "text-lg md:text-xl text-ctp-subtext0/90 font-light relative z-10 max-w-2xl mx-auto leading-relaxed",
                    "{description}"
                }

                // Decorative line
                div {
                    class: "w-32 h-0.5 bg-gradient-to-r from-ctp-mauve via-ctp-blue to-ctp-teal rounded-full mx-auto mt-8 opacity-70"
                }

                // Subtle animation dots
                div {
                    class: "flex justify-center space-x-2 mt-6",
                    div { class: "w-1.5 h-1.5 bg-ctp-mauve/60 rounded-full animate-pulse" }
                    div { class: "w-1.5 h-1.5 bg-ctp-blue/60 rounded-full animate-pulse" }
                    div { class: "w-1.5 h-1.5 bg-ctp-teal/60 rounded-full animate-pulse" }
                }
            }
        }
    }
}

#[component]
fn ToolRenderer(state: ToolsState, active_tab: String) -> Element {
    match active_tab.as_str() {
        "json" => rsx! {
            div { class: "animate-fade-in",
                JsonFormatter {
                    input: state.json_input,
                    output: state.json_output,
                    error: state.json_error
                }
            }
        },
        "text" => rsx! {
            div { class: "animate-fade-in",
                TextUtilities {
                    input: state.text_input,
                    word_count: state.word_count,
                    char_count: state.char_count
                }
            }
        },
        "qr" => rsx! {
            div { class: "animate-fade-in",
                QRGenerator {
                    input: state.qr_text,
                    qr_url: state.qr_url
                }
            }
        },
        "base64" => rsx! {
            div { class: "animate-fade-in",
                Base64Tool {
                    input: state.base64_input,
                    output: state.base64_output
                }
            }
        },
        _ => rsx! {
            div {
                class: "text-center text-ctp-subtext0 py-16 animate-fade-in",
                div { class: "text-6xl mb-4 opacity-50", "🔧" }
                p { class: "text-xl font-light", "Tool not found" }
            }
        },
    }
}

#[component]
fn PageFooter() -> Element {
    rsx! {
        div {
            class: "py-20 relative z-10",
            div {
                class: "text-center space-y-6",

                // Decorative separator
                div {
                    class: "flex justify-center",
                    div { class: "w-48 h-px bg-gradient-to-r from-transparent via-ctp-surface2/60 to-transparent" }
                }

                // Elegant footer text
                p {
                    class: "text-xs text-ctp-subtext0/60 font-light tracking-[0.2em] uppercase animate-fade-in",
                    "Crafted with precision & passion"
                }

                // Subtle dots
                div {
                    class: "flex justify-center space-x-3",
                    div { class: "w-1 h-1 bg-ctp-mauve/30 rounded-full" }
                    div { class: "w-1 h-1 bg-ctp-blue/30 rounded-full" }
                    div { class: "w-1 h-1 bg-ctp-teal/30 rounded-full" }
                }
            }
        }
    }
}
