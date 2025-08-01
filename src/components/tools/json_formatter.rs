use crate::components::tools::shared::*;
use dioxus::prelude::*;

/// JSON Formatter component
#[component]
pub fn JsonFormatter(
    input: Signal<String>,
    output: Signal<String>,
    error: Signal<String>,
) -> Element {
    let mut input = input;
    let mut output = output;
    let mut error = error;

    let mut format_json = move |_| {
        let json_str = input();
        error.set(String::new());

        if json_str.trim().is_empty() {
            output.set(String::new());
            return;
        }

        match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
                Ok(formatted) => output.set(formatted),
                Err(e) => error.set(format!("Formatting error: {}", e)),
            },
            Err(e) => {
                error.set(format!("JSON Parse Error: {}", e));
                output.set(String::new());
            }
        }
    };

    let clear_all = move |_| {
        input.set(String::new());
        output.set(String::new());
        error.set(String::new());
    };

    let copy_output = move |_| {
        if !output().is_empty() {
            // TODO: Implement actual clipboard functionality
            println!("📋 Copied JSON to clipboard");
        }
    };

    let left_content = rsx! {
        InputSection {
            label: "Input JSON".to_string(),
            helper_text: Some("Paste JSON to format & validate".to_string()),
            input: rsx! {
                ToolTextarea {
                    value: input(),
                    placeholder: "Paste JSON here...".to_string(),
                    rows: Some(6),
                    oninput: Some(
                        EventHandler::new(move |event: FormEvent| {
                            input.set(event.value());
                            format_json(());
                        }),
                    ),
                }
            },
        }
    };

    let right_content = rsx! {
        OutputSection {
            label: "Formatted JSON".to_string(),
            helper_text: Some("Formatted & validated output".to_string()),
            copy_button: if !output().is_empty() { Some(rsx! {
                CopyButton { text: output(), onclick: copy_output }
            }) } else { None },
            output: rsx! {
                ToolTextarea {
                    value: output(),
                    placeholder: "Formatted JSON will appear here...".to_string(),
                    rows: Some(6),
                    readonly: Some(true),
                }
            },
        }
    };

    let actions = rsx! {
        ActionButton {
            text: "Clear All".to_string(),
            onclick: clear_all,
            variant: Some("secondary".to_string()),
        }
    };

    rsx! {
        ToolGrid {
            left_content,
            right_content,
            actions: Some(actions),
            error_message: Some(error()),
        }
    }
}
