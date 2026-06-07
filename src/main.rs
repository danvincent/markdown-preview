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

    let temp_dir = std::env::temp_dir();
    let output_path = temp_dir.join(format!("{}.html", title));

    match fs::write(&output_path, &html_output) {
        Ok(_) => {
            println!("Preview generated: {}", output_path.display());

            if let Err(e) = open::that(&output_path) {
                eprintln!("Error opening browser: {}", e);
                eprintln!("Please open the file manually: {}", output_path.display());
            }
        }
        Err(e) => {
            eprintln!("Error writing HTML file: {}", e);
            std::process::exit(1);
        }
    }
}
