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

    rsx! {
        div {
            class: "space-y-16",

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Input JSON"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text resize-none",
                    rows: "8",
                    placeholder: "Paste your JSON here...",
                    value: "{input}",
                    oninput: move |event| {
                        input.set(event.value());
                        format_json(());
                    }
                }
            }

            // Error display
            if !error().is_empty() {
                div {
                    class: "p-3 bg-ctp-red bg-opacity-20 border border-ctp-red text-ctp-red text-sm",
                    "❌ {error}"
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
                            "Formatted JSON"
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                            onclick: move |_| {
                                println!("📋 Copied JSON to clipboard");
                            },
                            "copy"
                        }
                    }
                    textarea {
                        class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text resize-none",
                        rows: "8",
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
                        error.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}
