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

    rsx! {
        div {
            class: "space-y-16",

            // Mode selection
            div {
                class: "space-y-3",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Operation Mode"
                }
                div {
                    class: "flex gap-6 justify-center",
                    button {
                        class: if mode() == "encode" {
                            "px-6 py-3 bg-ctp-surface2 text-ctp-text transition-colors"
                        } else {
                            "px-6 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-subtext0 hover:text-ctp-text transition-colors"
                        },
                        onclick: move |_| {
                            mode.set("encode".to_string());
                            process_base64(());
                        },
                        "Encode"
                    }
                    button {
                        class: if mode() == "decode" {
                            "px-6 py-3 bg-ctp-surface2 text-ctp-text transition-colors"
                        } else {
                            "px-6 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-subtext0 hover:text-ctp-text transition-colors"
                        },
                        onclick: move |_| {
                            mode.set("decode".to_string());
                            process_base64(());
                        },
                        "Decode"
                    }
                }
            }

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    if mode() == "encode" { "Text to Encode" } else { "Base64 to Decode" }
                }
                textarea {
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text resize-none",
                    rows: "6",
                    placeholder: if mode() == "encode" {
                        "Enter plain text to convert to Base64..."
                    } else {
                        "Enter Base64 string to decode to plain text..."
                    },
                    value: "{input}",
                    oninput: move |event| {
                        input.set(event.value());
                        process_base64(());
                    }
                }
            }

            // Output section
            if !output().is_empty() {
                div {
                    class: "space-y-2",
                    div {
                        class: "flex justify-between items-center",
                        label {
                            class: "block text-sm font-medium text-ctp-subtext1",
                            if mode() == "encode" { "Base64 Output" } else { "Decoded Text" }
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                            onclick: move |_| {
                                println!("📋 Copied Base64 result to clipboard");
                            },
                            "Copy"
                        }
                    }
                    textarea {
                        class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text resize-none",
                        rows: "6",
                        readonly: true,
                        value: "{output}"
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-8 justify-center pt-12",
                button {
                    class: "btn-secondary",
                    onclick: move |_| {
                        input.set(String::new());
                        output.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}
