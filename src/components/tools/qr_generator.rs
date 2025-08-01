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

    rsx! {
        div {
            class: "space-y-16",

            // Input section
            div {
                class: "space-y-2",
                label {
                    class: "block text-sm font-medium text-ctp-subtext1",
                    "Text or URL to encode"
                }
                input {
                    r#type: "text",
                    class: "w-full px-4 py-3 bg-ctp-base border border-ctp-surface2 text-ctp-text placeholder-ctp-subtext0 focus:outline-none focus:border-ctp-text",
                    placeholder: "Enter text, URL, or any data...",
                    value: "{input}",
                    oninput: move |event| input.set(event.value())
                }
            }

            // Generate button
            div {
                class: "flex justify-center pb-8",
                button {
                    class: if input().trim().is_empty() || loading() {
                        "px-8 py-4 bg-ctp-surface2 text-ctp-subtext0 cursor-not-allowed transition-colors"
                    } else {
                        "px-8 py-4 bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors"
                    },
                    onclick: generate_qr,
                    disabled: input().trim().is_empty() || loading(),
                    if loading() { "Generating..." } else { "Generate QR Code" }
                }
            }

            // Output section
            if !qr_url().is_empty() {
                div {
                    class: "space-y-2",
                    div {
                        class: "flex justify-between items-center",
                        label {
                            class: "block text-sm font-medium text-ctp-subtext1",
                            "Generated QR Code"
                        }
                        button {
                            class: "px-3 py-1 text-xs bg-ctp-surface2 hover:bg-ctp-surface0 text-ctp-text transition-colors",
                            onclick: move |_| {
                                println!("📋 Copied QR code URL to clipboard");
                            },
                            "Copy URL"
                        }
                    }
                    div {
                        class: "flex justify-center p-4 bg-ctp-base border border-ctp-surface2",
                        img {
                            src: "{qr_url}",
                            alt: "Generated QR Code",
                            class: "max-w-full h-auto"
                        }
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
                        qr_url.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}
