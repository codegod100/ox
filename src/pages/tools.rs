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
    TabInfo { id: "json", icon: "{ }", title: "JSON", subtitle: "Format & Validate" },
    TabInfo { id: "text", icon: "Aa", title: "Text", subtitle: "Transform & Count" },
    TabInfo { id: "qr", icon: "⚡", title: "QR Code", subtitle: "Generate Codes" },
    TabInfo { id: "base64", icon: "🔒", title: "Base64", subtitle: "Encode & Decode" },
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

    fn set_active_tab(&mut self, tab_id: String) {
        self.active_tab.set(tab_id);
    }

    fn get_active_tab(&self) -> String {
        (self.active_tab)()
    }

    fn is_tab_active(&self, tab_id: &str) -> bool {
        self.get_active_tab() == tab_id
    }

    fn is_initialized(&self) -> bool {
        (self.is_initialized)()
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn get_button_class(is_active: bool) -> &'static str {
    if is_active {
        "p-6 bg-ctp-surface1 border-2 border-ctp-mauve rounded-lg"
    } else {
        "p-6 bg-ctp-surface0 border-2 border-ctp-surface2 rounded-lg hover:bg-ctp-surface1"
    }
}

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
        "json" => "Format, validate, and prettify your JSON data",
        "text" => "Transform text case and count characters & words",
        "qr" => "Generate QR codes for text, URLs, and any data",
        "base64" => "Encode and decode Base64 strings with ease",
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
        div { class: "min-h-screen bg-ctp-base",
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
        div { class: "min-h-screen pt-24 pb-16 flex items-center justify-center",
            div { class: "text-ctp-subtext0 text-lg", "Loading tools..." }
        }
    }
}

#[component]
fn PageHeader() -> Element {
    rsx! {
        div { class: "pt-24 pb-12",
            div { class: "container mx-auto px-6 text-center",
                h1 { class: "text-4xl font-light text-ctp-text mb-4", "Developer Tools" },
                p { class: "text-lg text-ctp-subtext0", "Essential utilities for developers" }
            }
        }
    }
}

#[component]
fn TabSelector(state: ToolsState) -> Element {
    let active_tab = state.get_active_tab();
    let mut active_tab_signal = state.active_tab;

    rsx! {
        div { class: "container mx-auto px-6 mb-12",
            div { class: "grid grid-cols-2 gap-4 max-w-4xl mx-auto",
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
    let button_class = get_button_class(is_active);
    let tab_id = tab.id.to_string();

    rsx! {
        button {
            key: "{tab.id}",
            class: "{button_class}",
            onclick: move |_| on_click.call(tab_id.clone()),
            TabButtonContent { tab: tab }
        }
    }
}

#[component]
fn TabButtonContent(tab: TabInfo) -> Element {
    rsx! {
        div { class: "text-center",
            div { class: "text-3xl mb-2", "{tab.icon}" },
            h3 { class: "font-semibold text-ctp-text mb-1", "{tab.title}" },
            p { class: "text-xs text-ctp-subtext0", "{tab.subtitle}" }
        }
    }
}

#[component]
fn ToolContent(state: ToolsState) -> Element {
    let active_tab = state.get_active_tab();

    rsx! {
        div { class: "container mx-auto px-6 pb-16",
            div { class: "max-w-7xl mx-auto",
                div { class: "bg-ctp-surface0 border border-ctp-surface2 rounded-lg p-8",
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
        div { class: "space-y-6",
            div { class: "text-center mb-8",
                h2 { class: "text-2xl font-light text-ctp-text mb-2", "{title}" },
                p { class: "text-ctp-subtext0", "{description}" }
            }
        }
    }
}

#[component]
fn ToolRenderer(state: ToolsState, active_tab: String) -> Element {
    match active_tab.as_str() {
        "json" => rsx! {
            JsonFormatter {
                input: state.json_input,
                output: state.json_output,
                error: state.json_error
            }
        },
        "text" => rsx! {
            TextUtilities {
                input: state.text_input,
                word_count: state.word_count,
                char_count: state.char_count
            }
        },
        "qr" => rsx! {
            QRGenerator {
                input: state.qr_text,
                qr_url: state.qr_url
            }
        },
        "base64" => rsx! {
            Base64Tool {
                input: state.base64_input,
                output: state.base64_output
            }
        },
        _ => rsx! {
            div { class: "text-center text-ctp-subtext0", "Unknown tool selected" }
        }
    }
}

#[component]
fn PageFooter() -> Element {
    rsx! {
        div { class: "border-t border-ctp-surface1 py-8",
            div { class: "container mx-auto px-6 text-center",
                p { class: "text-sm text-ctp-subtext0", "All processing happens locally in your browser. Your data never leaves your device." }
            }
        }
    }
}
