use crate::components::tools::shared::*;
use crate::server::generate_qr_code;
use dioxus::prelude::*;

/// QR Code Generator component
#[component]
pub fn QRGenerator(input: Signal<String>, qr_url: Signal<String>) -> Element {
    let mut input = input;
    let mut qr_url = qr_url;
    let mut loading = use_signal(|| false);

    let generate_qr = move |_| {
        let text = input();
        if text.trim().is_empty() {
            return;
        }

        loading.set(true);
        spawn(async move {
            match generate_qr_code(text).await {
                Ok(url) => qr_url.set(url),
                Err(e) => println!("QR generation error: {:?}", e),
            }
            loading.set(false);
        });
    };

    let clear_all = move |_| {
        input.set(String::new());
        qr_url.set(String::new());
    };

    let copy_url = move |_| {
        if !qr_url().is_empty() {
            // TODO: Implement actual clipboard functionality
            println!("📋 Copied QR code URL to clipboard");
        }
    };

    let download_qr = move |_: MouseEvent| {
        if !qr_url().is_empty() {
            // TODO: Implement actual download functionality
            println!("💾 Downloaded QR code image");
        }
    };

    let left_content = rsx! {
        InputSection {
            label: "Text or URL to encode".to_string(),
            helper_text: Some("Enter any text, URL, or data to generate a QR code".to_string()),
            input: rsx! {
                div { class: "space-y-4",
                    ToolInput {
                        value: input(),
                        placeholder: "Enter text, URL, or any data...".to_string(),
                        input_type: Some("text".to_string()),
                        oninput: Some(EventHandler::new(move |event: FormEvent| input.set(event.value()))),
                    }

                    // Generate button
                    ActionButton {
                        text: if loading() {
                            "⏳ Generating QR Code...".to_string()
                        } else {
                            "🔍 Generate QR Code".to_string()
                        },
                        onclick: generate_qr,
                        variant: Some("primary".to_string()),
                        disabled: Some(input().trim().is_empty() || loading()),
                    }
                }
            }
        }
    };

    let right_content = rsx! {
        OutputSection {
            label: "Generated QR Code".to_string(),
            helper_text: Some("Scan with your device's camera".to_string()),
            copy_button: if !qr_url().is_empty() {
                Some(rsx! {
                    div { class: "flex gap-2",
                        CopyButton {
                            text: qr_url(),
                            onclick: copy_url
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors rounded-md",
                            onclick: move |event| download_qr(event),
                            "💾 Download"
                        }
                    }
                })
            } else {
                None
            },
            output: rsx! {
                div { class: "h-full min-h-[300px] flex items-center justify-center p-6 bg-ctp-base border border-ctp-surface2 rounded-md",
                    if loading() {
                        div { class: "text-center",
                            div { class: "loading-pulse text-4xl mb-4", "⏳" }
                            div { class: "text-sm text-ctp-subtext1", "Generating QR code..." }
                        }
                    } else if !qr_url().is_empty() {
                        div { class: "text-center space-y-4",
                            img {
                                src: "{qr_url}",
                                alt: "Generated QR Code",
                                class: "max-w-full h-auto mx-auto border border-ctp-surface2 rounded-md bg-white p-4",
                            }
                            div { class: "text-xs text-ctp-subtext1",
                                "Scan with your device's camera"
                            }
                        }
                    } else {
                        div { class: "text-center text-ctp-subtext0",
                            div { class: "text-6xl mb-4 opacity-50", "📱" }
                            div { class: "text-sm", "QR code will appear here" }
                            div { class: "text-xs mt-2", "Enter text above and click Generate" }
                        }
                    }
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
        if !qr_url().is_empty() {
            ActionButton {
                text: "Download QR Code".to_string(),
                onclick: download_qr,
                variant: Some("secondary".to_string()),
            }
        }
    };

    rsx! {
        div { class: "space-y-8",
            // Main tool grid
            ToolGrid {
                left_content: left_content,
                right_content: right_content,
                actions: Some(actions),
            }

            // QR Code information section
            if !qr_url().is_empty() {
                div { class: "bg-ctp-surface0 border border-ctp-surface2 rounded-md p-6",
                    h3 { class: "text-lg font-medium text-ctp-text mb-4 text-center", "QR Code Information" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 text-sm",
                        div { class: "text-center p-3 bg-ctp-base rounded-md",
                            div { class: "font-medium text-ctp-text", "Content" }
                            div { class: "text-ctp-subtext1 mt-1 break-all",
                                {
                                    if input().len() > 30 {
                                        format!("{}...", &input()[..30])
                                    } else {
                                        input()
                                    }
                                }
                            }
                        }
                        div { class: "text-center p-3 bg-ctp-base rounded-md",
                            div { class: "font-medium text-ctp-text", "Length" }
                            div { class: "text-ctp-subtext1 mt-1", "{input().len()} characters" }
                        }
                        div { class: "text-center p-3 bg-ctp-base rounded-md",
                            div { class: "font-medium text-ctp-text", "Type" }
                            div { class: "text-ctp-subtext1 mt-1",
                                if input().starts_with("http://") || input().starts_with("https://") {
                                    "URL"
                                } else if input().contains("@") && input().contains(".") {
                                    "Email"
                                } else {
                                    "Text"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
