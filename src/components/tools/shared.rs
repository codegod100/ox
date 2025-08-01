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
        div { class: "space-y-8",

            // Error/Success messages
            if let Some(error) = props.error_message {
                if !error.is_empty() {
                    div { class: "p-4 bg-ctp-red/10 border border-ctp-red/40 text-ctp-red rounded-md animate-fade-in",
                        div { class: "flex items-center space-x-2",
                            span { "❌" }
                            span { "{error}" }
                        }
                    }
                }
            }

            if let Some(success) = props.success_message {
                if !success.is_empty() {
                    div { class: "p-4 bg-ctp-green/10 border border-ctp-green/40 text-ctp-green rounded-md animate-fade-in",
                        div { class: "flex items-center space-x-2",
                            span { "✅" }
                            span { "{success}" }
                        }
                    }
                }
            }

            // Main grid layout - responsive 2-column for larger tools
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

                // Input section
                div { class: "space-y-4", {props.left_content} }

                // Output section  
                div { class: "space-y-4", {props.right_content} }
            }

            // Actions row
            if let Some(actions) = props.actions {
                div { class: "flex flex-wrap gap-3 justify-center pt-4 mt-4 border-t border-ctp-surface1/50",
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

/// Standard input section
#[component]
pub fn InputSection(props: InputSectionProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            label { class: "block text-base font-medium text-ctp-text mb-2", "{props.label}" }
            {props.input}
            if let Some(helper) = props.helper_text {
                if !helper.is_empty() {
                    p { class: "text-sm text-ctp-subtext0/70 mt-2", "{helper}" }
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

/// Standard output section with copy functionality
#[component]
pub fn OutputSection(props: OutputSectionProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-between items-center mb-2",
                label { class: "block text-base font-medium text-ctp-text", "{props.label}" }
                if let Some(copy_btn) = props.copy_button {
                    {copy_btn}
                }
            }
            {props.output}
            if let Some(helper) = props.helper_text {
                if !helper.is_empty() {
                    p { class: "text-sm text-ctp-subtext0/70 mt-2", "{helper}" }
                }
            }
        }
    }
}

/// Clean textarea component
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

    let base_classes = "w-full resize-none font-mono text-sm bg-ctp-base border border-ctp-surface2 rounded-md p-4 text-ctp-text placeholder-ctp-subtext0 transition-colors focus:border-ctp-mauve focus:outline-none h-40";
    let readonly_classes = if readonly {
        " cursor-default bg-ctp-surface0"
    } else {
        ""
    };

    rsx! {
        textarea {
            class: "{base_classes}{readonly_classes}",
            rows: "{rows}",
            readonly,
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

/// Clean input component
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
            class: "w-full px-6 py-4 bg-ctp-base border border-ctp-surface2 rounded-md text-ctp-text placeholder-ctp-subtext0 transition-colors focus:outline-none focus:border-ctp-mauve",
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

/// Simple copy button
#[component]
pub fn CopyButton(text: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "px-3 py-2 bg-ctp-surface1 hover:bg-ctp-surface2 text-ctp-text text-sm rounded-md transition-colors flex items-center space-x-2 border border-ctp-surface2 hover:border-ctp-mauve",
            onclick: move |event| onclick.call(event),
            span { "📋" }
            span { "Copy" }
        }
    }
}

/// Simple action button with variants
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
                "px-4 py-2 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed rounded-md transition-colors"
            } else {
                "px-4 py-2 bg-ctp-mauve hover:bg-ctp-blue text-ctp-base rounded-md transition-colors"
            }
        }
        "danger" => {
            if disabled {
                "px-4 py-2 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed rounded-md transition-colors"
            } else {
                "px-4 py-2 bg-ctp-red hover:bg-ctp-red/80 text-ctp-base rounded-md transition-colors"
            }
        }
        _ => {
            if disabled {
                "px-4 py-2 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed rounded-md transition-colors"
            } else {
                "px-4 py-2 bg-ctp-surface1 hover:bg-ctp-surface2 text-ctp-text rounded-md transition-colors"
            }
        }
    };

    rsx! {
        button {
            class: "{button_class}",
            disabled,
            onclick: move |event| {
                if !disabled {
                    onclick.call(event);
                }
            },
            "{text}"
        }
    }
}

/// Simple statistics display
#[component]
pub fn StatsDisplay(stats: Vec<(String, String)>) -> Element {
    rsx! {
        div { class: "bg-ctp-surface0 border border-ctp-surface1 rounded-md p-4",
            div { class: "flex flex-wrap gap-6 justify-center",
                for (i , (label , value)) in stats.into_iter().enumerate() {
                    div { key: "{i}", class: "text-center",
                        div { class: "text-sm text-ctp-subtext0 uppercase mb-1", "{label}" }
                        div { class: "text-lg font-medium text-ctp-text", "{value}" }
                    }
                }
            }
        }
    }
}

/// Simple mode selector
#[component]
pub fn ModeSelector(
    current_mode: String,
    modes: Vec<(String, String)>, // (value, label) pairs
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "space-y-3 text-center",
            label { class: "block text-sm font-medium text-ctp-text", "Operation Mode" }
            div { class: "inline-flex gap-1 p-1 bg-ctp-surface0 border border-ctp-surface1 rounded-md",
                for (mode_value , mode_label) in modes {
                    {
                        let mode_value_clone = mode_value.clone();
                        rsx! {
                            button {
                                key: "{mode_value}",
                                class: if current_mode == mode_value { "px-3 py-2 bg-ctp-surface2 text-ctp-text font-medium rounded-sm transition-colors" } else { "px-3 py-2 text-ctp-text hover:text-ctp-mauve font-medium rounded-sm transition-colors hover:bg-ctp-surface1" },
                                onclick: move |_| on_change.call(mode_value_clone.clone()),
                                "{mode_label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
