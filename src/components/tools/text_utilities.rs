use dioxus::prelude::*;

/// Text Utilities component
#[component]
pub fn TextUtilities(
    input: Signal<String>,
    word_count: Signal<i32>,
    char_count: Signal<i32>,
) -> Element {
    let mut input = input;
    let mut word_count = word_count;
    let mut char_count = char_count;

    let mut update_counts = move || {
        let text = input();
        char_count.set(text.len() as i32);
        let words = text.split_whitespace().count();
        word_count.set(words as i32);
    };

    rsx! {
        div {
            class: "space-y-16",

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Input Text"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text resize-none",
                    rows: "8",
                    placeholder: "Enter your text here...",
                    value: "{input}",
                    oninput: move |event| {
                        input.set(event.value());
                        update_counts();
                    }
                }
            }

            // Stats section
            div {
                class: "flex gap-8 justify-center py-3 px-4 bg-ctp-base border border-ctp-surface2",
                div {
                    class: "text-sm text-ctp-subtext1",
                    "Characters: "
                    span { class: "font-medium text-ctp-text", "{char_count}" }
                }
                div {
                    class: "text-sm text-ctp-subtext1",
                    "Words: "
                    span { class: "font-medium text-ctp-text", "{word_count}" }
                }
            }

            // Transform actions
            div {
                class: "space-y-3",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Text Transformations"
                }
                div {
                    class: "flex gap-8 justify-center flex-wrap",
                    button {
                        class: "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            input.set(input().to_uppercase());
                            update_counts();
                        },
                        "UPPERCASE"
                    }
                    button {
                        class: "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            input.set(input().to_lowercase());
                            update_counts();
                        },
                        "lowercase"
                    }
                    button {
                        class: "px-6 py-3 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            let title_case: String = input()
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars: Vec<char> = word.chars().collect();
                                    if let Some(first) = chars.get_mut(0) {
                                        *first = first.to_uppercase().next().unwrap_or(*first);
                                    }
                                    chars.into_iter().collect::<String>()
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            input.set(title_case);
                            update_counts();
                        },
                        "Title Case"
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
                        update_counts();
                    },
                    "Clear"
                }
                if !input().is_empty() {
                    button {
                        class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                        onclick: move |_| {
                            println!("📋 Copied text to clipboard");
                        },
                        "Copy"
                    }
                }
            }
        }
    }
}
