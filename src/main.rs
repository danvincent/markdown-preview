use std::env;
use std::fs;
use std::path::Path;

use md_viewer::render_markdown;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown-file>", args[0]);
        std::process::exit(1);
    }
    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        eprintln!("Error: File '{}' not found", input_path.display());
        std::process::exit(1);
    }
    let markdown_input = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    let title = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Markdown Preview");
    let html_output = render_markdown(&markdown_input, title);

    // Start temporary HTTP server (serve once then quit)
    use tiny_http::{Server, Response, Header};
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    use std::{thread, time::Duration};

    // Bind to a random port on localhost
    let server = Server::http("127.0.0.1:0").expect("Cannot start server");
    let addr = server.server_addr();
    let url = format!("http://{}", addr);
    println!("Preview available at: {}", url);

    // Open browser
    if let Err(e) = open::that(&url) {
        eprintln!("Error opening browser: {}", e);
        eprintln!("Please open the address manually: {}", url);
    }

    let served = Arc::new(AtomicBool::new(false));
    let served_once = served.clone();
    let html_output = Arc::new(html_output);
    // Handle one request, then exit
    let server_handle = thread::spawn(move || {
        for request in server.incoming_requests() {
            let response = Response::from_string(html_output.as_str())
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
            let _ = request.respond(response);
            served_once.store(true, Ordering::SeqCst);
            break;
        }
    });

    // Wait until served once or a timeout
    let max_wait = Duration::from_secs(30);
    let sleep = Duration::from_millis(100);
    let mut waited = Duration::from_secs(0);
    while !served.load(Ordering::SeqCst) && waited < max_wait {
        thread::sleep(sleep);
        waited += sleep;
    }
    let _ = server_handle.join();
}
