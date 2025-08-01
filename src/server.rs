#[cfg(feature = "server")]
use dioxus::fullstack::prelude::*;

use dioxus::prelude::*;
use crate::types::NoCustomError;

#[server(EchoServer)]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    let start = std::time::Instant::now();
    println!(
        "📡 [{}] Server function called: echo_server with input: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        input
    );

    let response = format!("You said: {}", input);
    let elapsed = start.elapsed();
    println!(
        "📡 [{}] echo_server responding after {:?}: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        elapsed,
        response
    );
    Ok(response)
}

#[server(GetRandomCat)]
pub async fn get_random_cat() -> Result<String, ServerFnError> {
    println!("📡 Server function called: get_random_cat");
    // Using The Cat API for random cat images
    let response = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    let url = json[0]["url"].as_str().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("No cat URL found".to_string())
    })?;

    Ok(url.to_string())
}

#[server(GenerateBlogContent)]
pub async fn generate_blog_content(blog_id: i32) -> Result<(String, String), ServerFnError> {
    println!(
        "📡 Server function called: generate_blog_content with blog_id: {}",
        blog_id
    );
    use markov::Chain;

    // Sample corpus for training the Markov chain
    let corpus = "
        Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
        Dioxus is a modern Rust framework for building user interfaces. It provides a declarative way to build cross-platform applications.
        Web development has evolved significantly over the years. Modern frameworks make it easier to build interactive applications.
        Performance optimization is crucial for web applications. Users expect fast loading times and smooth interactions.
        Type safety helps prevent many common programming errors. Rust's ownership system ensures memory safety without garbage collection.
        Full-stack development allows developers to work on both frontend and backend components of an application.
        Server-side rendering improves initial page load times and search engine optimization.
        Component-based architecture promotes code reusability and maintainability.
        Reactive programming paradigms enable building responsive user interfaces.
        Security considerations are paramount in web application development.
        Code organization and project structure impact long-term maintainability.
        Testing strategies ensure application reliability and catch regressions early.
        Deployment processes should be automated and reliable for production environments.
        User experience design principles guide interface and interaction decisions.
        Accessibility features ensure applications are usable by everyone.
        Documentation helps other developers understand and contribute to projects.
    ";

    // Create and train the Markov chain
    let mut chain = Chain::new();
    chain.feed_str(corpus);

    // Use blog_id to determine which parts of the corpus to start from
    let sentences: Vec<&str> = corpus.split('.').collect();
    let start_idx = (blog_id as usize) % sentences.len();

    // Generate title starting from a specific sentence based on blog_id
    let title_start = sentences.get(start_idx).unwrap_or(&"Rust").trim();
    let title = if !title_start.is_empty() {
        let words: Vec<&str> = title_start.split_whitespace().take(4).collect();
        words.join(" ")
    } else {
        format!("Random Thoughts #{}", blog_id)
    };

    // Generate content using the Markov chain
    let content = if !chain.is_empty() {
        let mut generated = chain.generate_str_from_token("Rust");
        if generated.is_empty() {
            generated = chain.generate_str();
        }

        // Make it longer by appending more generated text
        for _ in 0..2 {
            let more = chain.generate_str();
            if !more.is_empty() {
                generated.push(' ');
                generated.push_str(&more);
            }
        }

        if generated.is_empty() {
            format!("This is a generated blog post using blog ID {} as a seed. The content is created using Markov chains to produce pseudo-random but coherent text about Rust and web development.", blog_id)
        } else {
            generated
        }
    } else {
        format!("This is a generated blog post using blog ID {} as a seed. The content is created using Markov chains to produce pseudo-random but coherent text.", blog_id)
    };

    Ok((title, content))
}

#[server(GenerateQRCode)]
pub async fn generate_qr_code(text: String) -> Result<String, ServerFnError> {
    let start = std::time::Instant::now();
    println!(
        "📡 [{}] Server function called: generate_qr_code with text: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        text
    );

    if text.trim().is_empty() {
        return Err(ServerFnError::ServerError(
            "Text cannot be empty".to_string(),
        ));
    }

    if text.len() > 2000 {
        return Err(ServerFnError::ServerError(
            "Text too long (max 2000 chars)".to_string(),
        ));
    }

    // Use a QR code generation service API (like qr-server.com)
    let encoded_text = urlencoding::encode(&text);
    let qr_url = format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=200x200&data={}",
        encoded_text
    );

    let elapsed = start.elapsed();
    println!(
        "📡 [{}] generate_qr_code responding after {:?} with URL: '{}'",
        chrono::Utc::now().format("%H:%M:%S%.3f"),
        elapsed,
        qr_url
    );
    Ok(qr_url)
}