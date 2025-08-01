use crate::components::tools::shared::*;
use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;

/// Base64 Encoder/Decoder component
#[component]
pub fn Base64Tool(input: Signal<String>, output: Signal<String>) -> Element {
    let mut input = input;
    let mut output = output;
    let mut mode = use_signal(|| "encode".to_string());

    let mut process_base64 = move |_| {
        let text = input();
        if text.trim().is_empty() {
            output.set(String::new());
            return;
        }

        if mode() == "encode" {
            let encoded = general_purpose::STANDARD.encode(text.as_bytes());
            output.set(encoded);
        } else {
            match general_purpose::STANDARD.decode(&text) {
                Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                    Ok(decoded_string) => output.set(decoded_string),
                    Err(_) => output.set("Error: Invalid UTF-8 in decoded data".to_string()),
                },
                Err(e) => output.set(format!("Decode error: {}", e)),
            }
        }
    };

    let clear_all = move |_| {
        input.set(String::new());
        output.set(String::new());
    };

    let copy_output = move |_| {
        if !output().is_empty() {
            // TODO: Implement actual clipboard functionality
            println!("📋 Copied Base64 result to clipboard");
        }
    };

    let modes = vec![
        ("encode".to_string(), "Encode".to_string()),
        ("decode".to_string(), "Decode".to_string()),
    ];

    let left_content = rsx! {
        InputSection {
            label: if mode() == "encode" {
                "Text to Encode".to_string()
            } else {
                "Base64 to Decode".to_string()
            },
            helper_text: Some(if mode() == "encode" {
                "Enter plain text to convert to Base64".to_string()
            } else {
                "Enter Base64 string to decode to plain text".to_string()
            }),
            input: rsx! {
                ToolTextarea {
                    value: input(),
                    placeholder: if mode() == "encode" {
                        "Enter plain text to convert to Base64...".to_string()
                    } else {
                        "Enter Base64 string to decode to plain text...".to_string()
                    },
                    rows: Some(12),
                    oninput: Some(EventHandler::new(move |event: FormEvent| {
                        input.set(event.value());
                        process_base64(());
                    })),
                }
            }
        }
    };

    let right_content = rsx! {
        OutputSection {
            label: if mode() == "encode" {
                "Base64 Output".to_string()
            } else {
                "Decoded Text".to_string()
            },
            helper_text: Some(if mode() == "encode" {
                "Base64 encoded result".to_string()
            } else {
                "Plain text decoded from Base64".to_string()
            }),
            copy_button: if !output().is_empty() {
                Some(rsx! {
                    CopyButton {
                        text: output(),
                        onclick: copy_output
                    }
                })
            } else {
                None
            },
            output: rsx! {
                ToolTextarea {
                    value: output(),
                    placeholder: "Output will appear here...".to_string(),
                    rows: Some(12),
                    readonly: Some(true),
                }
            }
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
        div { class: "space-y-8",
            // Mode selection
            ModeSelector {
                current_mode: mode(),
                modes: modes,
                on_change: move |new_mode| {
                    mode.set(new_mode);
                    process_base64(());
                }
            }

            // Main tool grid
            ToolGrid {
                left_content: left_content,
                right_content: right_content,
                actions: Some(actions),
            }
        }
    }
}
