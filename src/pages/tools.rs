use crate::components::tools::{Base64Tool, JsonFormatter, QRGenerator, TextUtilities};
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
        icon: "📄",
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
// VIEW COMPONENTS
// ============================================================================

#[component]
pub fn Tools() -> Element {
    let mut state = ToolsState::new();
    state.initialize();

    if !state.is_initialized() {
        return rsx! {
            LoadingView {}
        };
    }

    rsx! {
        div { class: "min-h-screen pt-20 pb-16",
            PageHeader {}
            TabSelector { state: state.clone() }
            ToolContent { state }
        }
    }
}

#[component]
fn LoadingView() -> Element {
    rsx! {
        div { class: "min-h-screen pt-20 pb-16 flex items-center justify-center",
            div { class: "flex flex-col items-center space-y-4 animate-fade-in",
                div { class: "w-8 h-8 border-2 border-ctp-surface2 border-t-ctp-mauve rounded-full animate-spin" }
                div { class: "text-ctp-subtext0 text-lg", "Loading tools..." }
            }
        }
    }
}

#[component]
fn PageHeader() -> Element {
    rsx! {
        div { class: "text-center mb-12 px-6",
            h1 { class: "text-4xl md:text-5xl font-bold text-ctp-text mb-4", "Developer Tools" }
            p { class: "text-lg text-ctp-subtext0 max-w-2xl mx-auto",
                "Professional-grade utilities for developers, designers, and digital creators"
            }
        }
    }
}

#[component]
fn TabSelector(state: ToolsState) -> Element {
    let active_tab = state.get_active_tab();
    let mut active_tab_signal = state.active_tab;

    rsx! {
        div { class: "max-w-4xl mx-auto px-6 mb-8",
            div { class: "grid grid-cols-2 lg:grid-cols-4 gap-3",
                for tab in TABS {
                    TabButton {
                        tab: tab.clone(),
                        is_active: active_tab == tab.id,
                        on_click: move |tab_id| active_tab_signal.set(tab_id),
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
        "bg-ctp-surface1 border border-ctp-mauve text-ctp-mauve"
    } else {
        "bg-ctp-surface0 border border-ctp-surface1 hover:border-ctp-surface2 hover:bg-ctp-surface1/50"
    };

    rsx! {
        button {
            key: "{tab.id}",
            class: "{button_classes} rounded-lg transition-all duration-200 cursor-pointer w-full min-h-[100px] flex flex-col items-center justify-center text-center p-4 hover:scale-105",
            onclick: move |_| on_click.call(tab_id.clone()),
            div { class: "text-2xl mb-2", "{tab.icon}" }
            h3 { class: "font-medium text-sm mb-1", "{tab.title}" }
            p { class: "text-xs text-ctp-subtext0/70", "{tab.subtitle}" }
        }
    }
}

#[component]
fn ToolContent(state: ToolsState) -> Element {
    let active_tab = state.get_active_tab();

    rsx! {
        div { class: "max-w-4xl mx-auto px-6",
            div { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-xl p-6",
                ToolRenderer { state, active_tab }
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
                    error: state.json_error,
                }
            }
        },
        "text" => rsx! {
            div { class: "animate-fade-in",
                TextUtilities {
                    input: state.text_input,
                    word_count: state.word_count,
                    char_count: state.char_count,
                }
            }
        },
        "qr" => rsx! {
            div { class: "animate-fade-in",
                QRGenerator { input: state.qr_text, qr_url: state.qr_url }
            }
        },
        "base64" => rsx! {
            div { class: "animate-fade-in",
                Base64Tool { input: state.base64_input, output: state.base64_output }
            }
        },
        _ => rsx! {
            div { class: "text-center text-ctp-subtext0 py-16 animate-fade-in",
                div { class: "text-6xl mb-4 opacity-50", "🔧" }
                p { class: "text-xl font-light", "Tool not found" }
            }
        },
    }
}

