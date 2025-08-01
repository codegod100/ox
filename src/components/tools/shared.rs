use dioxus::prelude::*;

/// Shared props for tool grid layout
#[derive(Props, Clone, PartialEq)]
pub struct ToolGridProps {
    /// Left column content (typically input)
    left_content: Element,
    /// Right column content (typically output)
    right_content: Element,
    /// Optional actions row at the bottom
    actions: Option<Element>,
    /// Optional error message to display at the top
    error_message: Option<String>,
    /// Optional success message to display at the top
    success_message: Option<String>,
}

/// Standard grid layout for tool inputs and outputs with modern glassmorphism
#[component]
pub fn ToolGrid(props: ToolGridProps) -> Element {
    rsx! {
        div { class: "space-y-8",

            // Error/Success messages with modern styling
            if let Some(error) = props.error_message {
                if !error.is_empty() {
                    div {
                        class: "relative overflow-hidden p-6 bg-gradient-to-r from-ctp-red/20 via-ctp-red/10 to-ctp-red/20 border border-ctp-red/40 backdrop-blur-xl text-ctp-red rounded-2xl shadow-lg animate-slide-in",
                        div { class: "absolute inset-0 bg-gradient-to-r from-ctp-red/5 to-transparent" }
                        div { class: "relative z-10 flex items-center space-x-3",
                            div { class: "text-2xl animate-pulse", "❌" }
                            span { class: "font-medium tracking-wide", "{error}" }
                        }
                    }
                }
            }

            if let Some(success) = props.success_message {
                if !success.is_empty() {
                    div {
                        class: "relative overflow-hidden p-6 bg-gradient-to-r from-ctp-green/20 via-ctp-green/10 to-ctp-green/20 border border-ctp-green/40 backdrop-blur-xl text-ctp-green rounded-2xl shadow-lg animate-slide-in",
                        div { class: "absolute inset-0 bg-gradient-to-r from-ctp-green/5 to-transparent" }
                        div { class: "relative z-10 flex items-center space-x-3",
                            div { class: "text-2xl animate-pulse", "✅" }
                            span { class: "font-medium tracking-wide", "{success}" }
                        }
                    }
                }
            }

            // Main grid layout - responsive 2-column grid with enhanced styling
            div {
                class: "grid grid-cols-1 xl:grid-cols-2 gap-12 items-start",

                // Left column (Input)
                div { class: "space-y-4 animate-slide-in", {props.left_content} }

                // Right column (Output)
                div { class: "space-y-4 animate-slide-in", {props.right_content} }
            }

            // Actions row with glassmorphism
            if let Some(actions) = props.actions {
                div {
                    class: "flex gap-6 justify-center pt-8 mt-8 border-t border-ctp-surface2/30 backdrop-blur-sm",
                    div { class: "flex flex-wrap gap-4 justify-center",
                        {actions}
                    }
                }
            }
        }
    }
}

/// Props for input section component
#[derive(Props, Clone, PartialEq)]
pub struct InputSectionProps {
    /// Label text for the input
    label: String,
    /// Input element (textarea, input, etc.)
    input: Element,
    /// Optional helper text
    helper_text: Option<String>,
}

/// Standard input section with modern styling
#[component]
pub fn InputSection(props: InputSectionProps) -> Element {
    rsx! {
        div { class: "space-y-4 group",
            label {
                class: "block text-lg font-medium text-ctp-text tracking-wide group-hover:text-ctp-mauve transition-colors duration-300",
                "{props.label}"
            }
            div { class: "relative",
                {props.input}
                // Subtle glow effect
                div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/5 via-transparent to-ctp-blue/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none" }
            }
            if let Some(helper) = props.helper_text {
                if !helper.is_empty() {
                    p {
                        class: "text-sm text-ctp-subtext0/80 font-light tracking-wide leading-relaxed",
                        "{helper}"
                    }
                }
            }
        }
    }
}

/// Props for output section component
#[derive(Props, Clone, PartialEq)]
pub struct OutputSectionProps {
    /// Label text for the output
    label: String,
    /// Output element (textarea, div, etc.)
    output: Element,
    /// Optional copy button
    copy_button: Option<Element>,
    /// Optional helper text
    helper_text: Option<String>,
}

/// Standard output section with modern styling and copy functionality
#[component]
pub fn OutputSection(props: OutputSectionProps) -> Element {
    rsx! {
        div { class: "space-y-4 group",
            div { class: "flex justify-between items-center",
                label {
                    class: "block text-lg font-medium text-ctp-text tracking-wide group-hover:text-ctp-blue transition-colors duration-300",
                    "{props.label}"
                }
                if let Some(copy_btn) = props.copy_button {
                    div { class: "animate-fade-in",
                        {copy_btn}
                    }
                }
            }
            div { class: "relative",
                {props.output}
                // Success indicator glow
                div { class: "absolute inset-0 bg-gradient-to-r from-ctp-blue/5 via-transparent to-ctp-teal/5 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none" }
            }
            if let Some(helper) = props.helper_text {
                if !helper.is_empty() {
                    p {
                        class: "text-sm text-ctp-subtext0/80 font-light tracking-wide leading-relaxed",
                        "{helper}"
                    }
                }
            }
        }
    }
}

/// Modern textarea component with glassmorphism styling
#[component]
pub fn ToolTextarea(
    value: String,
    placeholder: String,
    rows: Option<u32>,
    readonly: Option<bool>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let rows = rows.unwrap_or(12);
    let readonly = readonly.unwrap_or(false);

    let base_classes = "w-full resize-none font-mono text-base leading-relaxed bg-gradient-to-br from-ctp-base/80 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl p-6 text-ctp-text placeholder-ctp-subtext0/60 transition-all duration-500 focus:border-ctp-mauve/60 focus:shadow-glow focus:bg-ctp-base/90 focus:outline-none";
    let readonly_classes = if readonly {
        " cursor-default bg-ctp-surface0/40"
    } else {
        " hover:border-ctp-text/40"
    };

    rsx! {
        div { class: "relative group",
            textarea {
                class: "{base_classes}{readonly_classes}",
                rows: "{rows}",
                readonly: readonly,
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |event| {
                    if let Some(handler) = oninput {
                        handler.call(event);
                    }
                },
            }
            // Subtle border glow effect
            div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/10 via-ctp-blue/10 to-ctp-teal/10 rounded-2xl opacity-0 group-focus-within:opacity-100 transition-opacity duration-500 pointer-events-none blur-sm" }
        }
    }
}

/// Modern input component with glassmorphism styling
#[component]
pub fn ToolInput(
    value: String,
    placeholder: String,
    input_type: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());

    rsx! {
        div { class: "relative group",
            input {
                r#type: "{input_type}",
                class: "w-full px-6 py-4 bg-gradient-to-br from-ctp-base/80 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl text-ctp-text text-lg placeholder-ctp-subtext0/60 transition-all duration-500 focus:outline-none focus:border-ctp-mauve/60 focus:shadow-glow focus:bg-ctp-base/90 hover:border-ctp-text/40",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |event| {
                    if let Some(handler) = oninput {
                        handler.call(event);
                    }
                },
            }
            // Floating border effect
            div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/10 via-ctp-blue/10 to-ctp-teal/10 rounded-2xl opacity-0 group-focus-within:opacity-100 transition-opacity duration-500 pointer-events-none blur-sm" }
        }
    }
}

/// Modern copy button with sleek glassmorphism styling
#[component]
pub fn CopyButton(text: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "group relative overflow-hidden px-6 py-3 bg-gradient-to-r from-ctp-surface1/60 to-ctp-surface2/40 border border-ctp-surface2/50 backdrop-blur-xl hover:border-ctp-mauve/60 text-ctp-text hover:text-ctp-mauve transition-all duration-300 rounded-xl font-medium tracking-wide shadow-lg hover:shadow-glow hover:scale-105",
            onclick: move |event| onclick.call(event),

            // Shine effect
            div { class: "absolute inset-0 bg-shimmer-gradient opacity-0 group-hover:opacity-100 transition-opacity duration-500 group-hover:animate-shine" }

            span { class: "relative z-10 flex items-center space-x-2",
                span { class: "text-lg", "📋" }
                span { "Copy" }
            }
        }
    }
}

/// Modern action button with variant styling and glassmorphism
#[component]
pub fn ActionButton(
    text: String,
    onclick: EventHandler<MouseEvent>,
    variant: Option<String>,
    disabled: Option<bool>,
) -> Element {
    let variant = variant.unwrap_or_else(|| "secondary".to_string());
    let disabled = disabled.unwrap_or(false);

    let button_class = match variant.as_str() {
        "primary" => {
            if disabled {
                "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-surface2/50 to-ctp-surface1/50 border border-ctp-surface2/30 backdrop-blur-xl text-ctp-subtext0/50 cursor-not-allowed rounded-2xl font-medium tracking-wide transition-all duration-300"
            } else {
                "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-mauve/80 to-ctp-blue/60 border border-ctp-mauve/60 backdrop-blur-xl hover:from-ctp-mauve hover:to-ctp-blue text-ctp-base hover:text-ctp-crust transition-all duration-300 rounded-2xl font-medium tracking-wide shadow-glow hover:shadow-glow-lg hover:scale-105"
            }
        }
        "danger" => {
            if disabled {
                "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-surface2/50 to-ctp-surface1/50 border border-ctp-surface2/30 backdrop-blur-xl text-ctp-subtext0/50 cursor-not-allowed rounded-2xl font-medium tracking-wide transition-all duration-300"
            } else {
                "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-red/80 to-ctp-maroon/60 border border-ctp-red/60 backdrop-blur-xl hover:from-ctp-red hover:to-ctp-maroon text-ctp-base hover:text-ctp-crust transition-all duration-300 rounded-2xl font-medium tracking-wide shadow-lg hover:shadow-xl hover:scale-105"
            }
        }
        _ => {
            if disabled {
                "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-surface2/50 to-ctp-surface1/50 border border-ctp-surface2/30 backdrop-blur-xl text-ctp-subtext0/50 cursor-not-allowed rounded-2xl font-medium tracking-wide transition-all duration-300"
            } else {
                "group relative overflow-hidden px-8 py-4 bg-gradient-to-r from-ctp-surface1/60 to-ctp-surface2/40 border border-ctp-surface2/50 backdrop-blur-xl hover:border-ctp-text/60 text-ctp-text hover:text-ctp-mauve transition-all duration-300 rounded-2xl font-medium tracking-wide shadow-lg hover:shadow-glow hover:scale-105"
            }
        }
    };

    rsx! {
        button {
            class: "{button_class}",
            disabled: disabled,
            onclick: move |event| {
                if !disabled {
                    onclick.call(event);
                }
            },

            // Shine effect for non-disabled buttons
            if !disabled {
                div { class: "absolute inset-0 bg-shimmer-gradient opacity-0 group-hover:opacity-100 transition-opacity duration-500 group-hover:animate-shine" }
            }

            span { class: "relative z-10", "{text}" }
        }
    }
}

/// Modern statistics display with glassmorphism and animations
#[component]
pub fn StatsDisplay(stats: Vec<(String, String)>) -> Element {
    rsx! {
        div {
            class: "relative overflow-hidden bg-gradient-to-br from-ctp-surface0/60 via-ctp-surface1/40 to-ctp-surface0/60 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl p-8 shadow-glass animate-scale-in",

            // Background glow
            div { class: "absolute inset-0 bg-gradient-to-r from-ctp-mauve/5 via-ctp-blue/5 to-ctp-teal/5 opacity-50" }

            div {
                class: "relative z-10 flex flex-wrap gap-8 justify-center",
                for (i, (label, value)) in stats.into_iter().enumerate() {
                    div {
                        key: "{i}",
                        class: "group text-center animate-fade-in",
                        style: "animation-delay: {i * 100}ms",

                        div { class: "text-sm text-ctp-subtext1/80 font-light tracking-wide uppercase mb-2",
                            "{label}"
                        }
                        div { class: "text-2xl font-medium text-ctp-text group-hover:text-ctp-mauve transition-colors duration-300 tracking-wide",
                            "{value}"
                        }
                        div { class: "w-8 h-0.5 bg-gradient-to-r from-ctp-mauve to-ctp-blue rounded-full mx-auto mt-2 opacity-0 group-hover:opacity-100 transition-opacity duration-300" }
                    }
                }
            }
        }
    }
}

/// Modern mode selector with sleek toggle styling
#[component]
pub fn ModeSelector(
    current_mode: String,
    modes: Vec<(String, String)>, // (value, label) pairs
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "space-y-6 text-center",
            label {
                class: "block text-lg font-medium text-ctp-text tracking-wide",
                "Operation Mode"
            }
            div { class: "inline-flex gap-2 p-2 bg-gradient-to-r from-ctp-surface0/60 to-ctp-surface1/40 border border-ctp-surface2/50 backdrop-blur-xl rounded-2xl shadow-inner-glow",
                for (mode_value, mode_label) in modes {
                    button {
                        key: "{mode_value}",
                        class: if current_mode == mode_value {
                            "relative px-6 py-3 bg-gradient-to-r from-ctp-mauve to-ctp-blue text-ctp-base font-medium rounded-xl shadow-glow transition-all duration-300 scale-105"
                        } else {
                            "relative px-6 py-3 text-ctp-text hover:text-ctp-mauve font-medium rounded-xl transition-all duration-300 hover:bg-ctp-surface1/40"
                        },
                        onclick: move |_| on_change.call(mode_value.clone()),

                        // Active state shine effect
                        if current_mode == mode_value {
                            div { class: "absolute inset-0 bg-shimmer-gradient opacity-30 rounded-xl animate-shine" }
                        }

                        span { class: "relative z-10", "{mode_label}" }
                    }
                }
            }
        }
    }
}
