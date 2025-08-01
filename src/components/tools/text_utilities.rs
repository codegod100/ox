use crate::components::tools::shared::*;
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

    let copy_text = move |_| {
        if !input().is_empty() {
            // TODO: Implement actual clipboard functionality
            println!("📋 Copied text to clipboard");
        }
    };

    let clear_all = move |_| {
        input.set(String::new());
        update_counts();
    };

    let left_content = rsx! {
        InputSection {
            label: "Input Text".to_string(),
            helper_text: Some("Enter text to analyze & transform".to_string()),
            input: rsx! {
                ToolTextarea {
                    value: input(),
                    placeholder: "Enter text here...".to_string(),
                    rows: Some(6),
                    oninput: Some(
                        EventHandler::new(move |event: FormEvent| {
                            input.set(event.value());
                            update_counts();
                        }),
                    ),
                }
            },
        }
    };

    let right_content = rsx! {
        OutputSection {
            label: "Text Statistics".to_string(),
            helper_text: Some("Real-time analysis of your text".to_string()),
            copy_button: if !input().is_empty() { Some(rsx! {
                CopyButton { text: input(), onclick: copy_text }
            }) } else { None },
            output: rsx! {
                div { class: "h-full bg-ctp-base border border-ctp-surface2 rounded-md p-4 space-y-4",
                    // Main statistics
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "text-center p-4 bg-ctp-surface0 rounded-md",
                            div { class: "text-2xl font-bold text-ctp-text", "{char_count}" }
                            div { class: "text-sm text-ctp-subtext1", "Characters" }
                        }
                        div { class: "text-center p-4 bg-ctp-surface0 rounded-md",
                            div { class: "text-2xl font-bold text-ctp-text", "{word_count}" }
                            div { class: "text-sm text-ctp-subtext1", "Words" }
                        }
                    }
                    // Additional text metrics
                    if !input().is_empty() {
                        div { class: "pt-4 border-t border-ctp-surface2 space-y-3",
                            div { class: "grid grid-cols-1 gap-2 text-sm",
                                div { class: "flex justify-between",
                                    span { class: "text-ctp-subtext1", "Lines:" }
                                    span { class: "text-ctp-text font-medium", "{input().lines().count()}" }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-ctp-subtext1", "Paragraphs:" }
                                    span { class: "text-ctp-text font-medium",
                                        "{input().split(\"\\n\\n\").filter(|p| !p.trim().is_empty()).count()}"
                                    }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-ctp-subtext1", "Characters (no spaces):" }
                                    span { class: "text-ctp-text font-medium",
                                        "{input().chars().filter(|c| !c.is_whitespace()).count()}"
                                    }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-ctp-subtext1", "Average word length:" }
                                    span { class: "text-ctp-text font-medium",
                                        {
                                            if word_count() > 0 {
                                                format!(
                                                    "{:.1}",
                                                    input().chars().filter(|c| !c.is_whitespace()).count() as f32
                                                        / word_count() as f32,
                                                )
                                            } else {
                                                "0".to_string()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "text-center text-ctp-subtext0 py-8",
                            div { class: "text-4xl mb-4 opacity-50", "📊" }
                            div { class: "text-sm", "Statistics will appear here" }
                            div { class: "text-xs mt-2", "Enter text to see analysis" }
                        }
                    }
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
        div { class: "space-y-4",
            // Main tool grid
            ToolGrid { left_content, right_content, actions: Some(actions) }

            // Text transformation tools
            div { class: "border-t border-ctp-surface1 pt-4",
                div { class: "grid grid-cols-3 gap-2",
                    ActionButton {
                        text: "UPPER".to_string(),
                        onclick: move |_| {
                            input.set(input().to_uppercase());
                            update_counts();
                        },
                        variant: Some("secondary".to_string()),
                        disabled: Some(input().is_empty()),
                    }
                    ActionButton {
                        text: "lower".to_string(),
                        onclick: move |_| {
                            input.set(input().to_lowercase());
                            update_counts();
                        },
                        variant: Some("secondary".to_string()),
                        disabled: Some(input().is_empty()),
                    }
                    ActionButton {
                        text: "Title".to_string(),
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
                        variant: Some("secondary".to_string()),
                        disabled: Some(input().is_empty()),
                    }
                }
            }
        }
    }
}
