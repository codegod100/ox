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

/// Standard grid layout for tool inputs and outputs
#[component]
pub fn ToolGrid(props: ToolGridProps) -> Element {
    rsx! {
        div { class: "space-y-6",

            // Error/Success messages
            if let Some(error) = props.error_message {
                if !error.is_empty() {
                    div {
                        class: "p-4 bg-ctp-red bg-opacity-20 border border-ctp-red text-ctp-red text-sm rounded-md",
                        "❌ {error}"
                    }
                }
            }

            if let Some(success) = props.success_message {
                if !success.is_empty() {
                    div {
                        class: "p-4 bg-ctp-green bg-opacity-20 border border-ctp-green text-ctp-green text-sm rounded-md",
                        "✅ {success}"
                    }
                }
            }

            // Main grid layout - responsive 2-column grid
            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",

                // Left column (Input)
                div { class: "space-y-3", {props.left_content} }

                // Right column (Output)
                div { class: "space-y-3", {props.right_content} }
            }

            // Actions row
            if let Some(actions) = props.actions {
                div { class: "flex gap-4 justify-center pt-6 border-t border-ctp-surface2",
                    {actions}
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

/// Standard input section with label and helper text
#[component]
pub fn InputSection(props: InputSectionProps) -> Element {
    rsx! {
        div { class: "space-y-3",
            label {
                class: "block text-sm font-medium text-ctp-subtext1",
                "{props.label}"
            }
            {props.input}
            if let Some(helper) = props.helper_text {
                if !helper.is_empty() {
                    p {
                        class: "text-xs text-ctp-subtext0 mt-1",
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

/// Standard output section with label and optional copy button
#[component]
pub fn OutputSection(props: OutputSectionProps) -> Element {
    rsx! {
        div { class: "space-y-3",
            div { class: "flex justify-between items-center",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "{props.label}"
                }
                if let Some(copy_btn) = props.copy_button {
                    {copy_btn}
                }
            }
            {props.output}
            if let Some(helper) = props.helper_text {
                if !helper.is_empty() {
                    p {
                        class: "text-xs text-ctp-subtext0 mt-1",
                        "{helper}"
                    }
                }
            }
        }
    }
}

/// Standard textarea component with consistent styling
#[component]
pub fn ToolTextarea(
    value: String,
    placeholder: String,
    rows: Option<u32>,
    readonly: Option<bool>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let rows = rows.unwrap_or(10);
    let readonly = readonly.unwrap_or(false);

    rsx! {
        textarea {
            class: "w-full resize-none font-mono text-sm leading-relaxed",
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
    }
}

/// Standard input component with consistent styling
#[component]
pub fn ToolInput(
    value: String,
    placeholder: String,
    input_type: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());

    rsx! {
        input {
            r#type: "{input_type}",
            class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text rounded-md transition-colors",
            placeholder: "{placeholder}",
            value: "{value}",
            oninput: move |event| {
                if let Some(handler) = oninput {
                    handler.call(event);
                }
            },
        }
    }
}

/// Standard copy button component
#[component]
pub fn CopyButton(text: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors rounded-md",
            onclick: move |event| onclick.call(event),
            "📋 Copy"
        }
    }
}

/// Standard action button component
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
                "px-6 py-3 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed transition-colors rounded-md"
            } else {
                "px-6 py-3 bg-ctp-mauve hover:bg-ctp-mauve text-ctp-base hover:text-ctp-base transition-colors rounded-md"
            }
        }
        "danger" => {
            if disabled {
                "px-6 py-3 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed transition-colors rounded-md"
            } else {
                "px-6 py-3 bg-ctp-red hover:bg-ctp-maroon text-ctp-base transition-colors rounded-md"
            }
        }
        _ => {
            if disabled {
                "px-6 py-3 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed transition-colors rounded-md"
            } else {
                "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors rounded-md"
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
            "{text}"
        }
    }
}

/// Statistics display component for showing counts, stats, etc.
#[component]
pub fn StatsDisplay(stats: Vec<(String, String)>) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-6 justify-center py-4 px-6 bg-ctp-base border border-ctp-surface2 rounded-md",
            for (label, value) in stats {
                div { class: "text-sm text-ctp-subtext1",
                    "{label}: "
                    span { class: "font-medium text-ctp-text", "{value}" }
                }
            }
        }
    }
}

/// Mode selector component for tools with multiple modes (encode/decode, etc.)
#[component]
pub fn ModeSelector(
    current_mode: String,
    modes: Vec<(String, String)>, // (value, label) pairs
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "space-y-4",
            label {
                class: "block text-sm font-medium text-ctp-subtext1 text-center",
                "Operation Mode"
            }
            div { class: "flex gap-3 justify-center flex-wrap",
                for (mode_value, mode_label) in modes {
                    button {
                        class: if current_mode == mode_value {
                            "px-4 py-2 bg-ctp-mauve text-ctp-base transition-colors rounded-md"
                        } else {
                            "px-4 py-2 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors rounded-md"
                        },
                        onclick: move |_| on_change.call(mode_value.clone()),
                        "{mode_label}"
                    }
                }
            }
        }
    }
}
