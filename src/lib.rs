use pulldown_cmark::{html, Options, Parser};

/// GitHub-flavored markdown CSS styles (light + dark mode).
pub const CSS_STYLES: &str = r#"
        :root {
            --bg-color: #ffffff;
            --text-color: #24292f;
            --border-color: #d0d7de;
            --code-bg: #f6f8fa;
            --link-color: #0969da;
            --blockquote-color: #57606a;
            --blockquote-border: #d0d7de;
        }

        @media (prefers-color-scheme: dark) {
            :root {
                --bg-color: #0d1117;
                --text-color: #c9d1d9;
                --border-color: #30363d;
                --code-bg: #161b22;
                --link-color: #58a6ff;
                --blockquote-color: #8b949e;
                --blockquote-border: #30363d;
            }
        }

        * {
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            font-size: 16px;
            line-height: 1.6;
            color: var(--text-color);
            background-color: var(--bg-color);
            margin: 0;
            padding: 0;
        }

        .markdown-body {
            max-width: 900px;
            margin: 0 auto;
            padding: 40px 32px;
        }

        h1, h2, h3, h4, h5, h6 {
            margin-top: 24px;
            margin-bottom: 16px;
            font-weight: 600;
            line-height: 1.25;
        }

        h1 { font-size: 2em; border-bottom: 1px solid var(--border-color); padding-bottom: 0.3em; }
        h2 { font-size: 1.5em; border-bottom: 1px solid var(--border-color); padding-bottom: 0.3em; }
        h3 { font-size: 1.25em; }
        h4 { font-size: 1em; }
        h5 { font-size: 0.875em; }
        h6 { font-size: 0.85em; color: var(--blockquote-color); }

        p {
            margin-top: 0;
            margin-bottom: 16px;
        }

        a {
            color: var(--link-color);
            text-decoration: none;
        }

        a:hover {
            text-decoration: underline;
        }

        code {
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
            font-size: 85%;
            background-color: var(--code-bg);
            padding: 0.2em 0.4em;
            border-radius: 6px;
        }

        pre {
            background-color: var(--code-bg);
            border-radius: 6px;
            padding: 16px;
            overflow: auto;
            line-height: 1.45;
        }

        pre code {
            background-color: transparent;
            padding: 0;
            font-size: 100%;
        }

        blockquote {
            margin: 0 0 16px;
            padding: 0 16px;
            color: var(--blockquote-color);
            border-left: 4px solid var(--blockquote-border);
        }

        ul, ol {
            margin-top: 0;
            margin-bottom: 16px;
            padding-left: 2em;
        }

        li + li {
            margin-top: 0.25em;
        }

        img {
            max-width: 100%;
            box-sizing: content-box;
            background-color: var(--bg-color);
        }

        table {
            border-spacing: 0;
            border-collapse: collapse;
            margin-bottom: 16px;
            width: 100%;
            overflow: auto;
        }

        table th, table td {
            padding: 6px 13px;
            border: 1px solid var(--border-color);
        }

        table tr:nth-child(2n) {
            background-color: var(--code-bg);
        }

        hr {
            height: 0.25em;
            padding: 0;
            margin: 24px 0;
            background-color: var(--border-color);
            border: 0;
        }

        .task-list-item {
            list-style-type: none;
        }

        .task-list-item input {
            margin: 0 0.2em 0.25em -1.6em;
            vertical-align: middle;
        }
"#;

/// Convert GitHub-flavored markdown to an HTML fragment.
pub fn markdown_to_html_fragment(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Build a complete HTML document from rendered HTML content.
pub fn build_html(title: &str, body_html: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        {}
    </style>
</head>
<body>
    <div class="markdown-body">
        {}
    </div>
</body>
</html>"#,
        title, CSS_STYLES, body_html
    )
}

/// Convert markdown to a complete HTML preview document.
pub fn render_markdown(markdown: &str, title: &str) -> String {
    let body_html = markdown_to_html_fragment(markdown);
    build_html(title, &body_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html_fragment_converts_paragraphs() {
        let input = "Hello **world**";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<p>"));
        assert!(output.contains("<strong>world</strong>"));
        assert!(output.contains("</p>"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_headings() {
        let input = "# Heading 1\n\n## Heading 2";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<h1>"));
        assert!(output.contains("Heading 1"));
        assert!(output.contains("<h2>"));
        assert!(output.contains("Heading 2"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_code_blocks() {
        let input = "```rust\nfn main() {}\n```";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<pre"));
        assert!(output.contains("<code"));
        assert!(output.contains("fn main()"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_inline_code() {
        let input = "Use `println!` macro";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<code>"));
        assert!(output.contains("println!"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_links() {
        let input = "[Click here](https://example.com)";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<a href="));
        assert!(output.contains("https://example.com"));
        assert!(output.contains("Click here"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_blockquotes() {
        let input = "> This is a quote";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<blockquote>"));
        assert!(output.contains("This is a quote"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_unordered_lists() {
        let input = "- Item 1\n- Item 2";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<ul>"));
        assert!(output.contains("<li>"));
        assert!(output.contains("Item 1"));
        assert!(output.contains("Item 2"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_ordered_lists() {
        let input = "1. First\n2. Second";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<ol>"));
        assert!(output.contains("<li>"));
        assert!(output.contains("First"));
        assert!(output.contains("Second"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_tables() {
        let input = "| A | B |\n|---|---|\n| 1 | 2 |";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<table>"));
        assert!(output.contains("<th>A</th>"));
        assert!(output.contains("<td>1</td>"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_horizontal_rule() {
        let input = "---";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<hr"));
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_strikethrough() {
        let input = "~~deleted~~";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<del>"));
        assert!(output.contains("deleted"));
    }

    #[test]
    fn test_markdown_to_html_fragment_empty_input() {
        let output = markdown_to_html_fragment("");
        assert_eq!(output, "");
    }

    #[test]
    fn test_markdown_to_html_fragment_converts_images() {
        let input = "![alt text](image.png)";
        let output = markdown_to_html_fragment(input);
        assert!(output.contains("<img"));
        assert!(output.contains("src=\"image.png\""));
        assert!(output.contains("alt=\"alt text\""));
    }

    #[test]
    fn test_build_html_contains_doctype() {
        let html = build_html("Test", "<p>content</p>");
        assert!(html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn test_build_html_contains_title() {
        let html = build_html("My Title", "<p>content</p>");
        assert!(html.contains("<title>My Title</title>"));
    }

    #[test]
    fn test_build_html_contains_css_styles() {
        let html = build_html("Test", "<p>content</p>");
        assert!(html.contains("--bg-color"));
        assert!(html.contains("prefers-color-scheme: dark"));
    }

    #[test]
    fn test_build_html_contains_body_content() {
        let html = build_html("Test", "<p>Hello</p>");
        assert!(html.contains("<p>Hello</p>"));
        assert!(html.contains("class=\"markdown-body\""));
    }

    #[test]
    fn test_build_html_has_valid_structure() {
        let html = build_html("Test", "<p>content</p>");
        assert!(html.contains("<html"));
        assert!(html.contains("<head>"));
        assert!(html.contains("<body>"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn test_render_markdown_combines_fragment_and_template() {
        let input = "# Hello";
        let output = render_markdown(input, "My Doc");
        assert!(output.starts_with("<!DOCTYPE html>"));
        assert!(output.contains("<title>My Doc</title>"));
        assert!(output.contains("<h1>Hello</h1>"));
        assert!(output.contains("--bg-color"));
    }

    #[test]
    fn test_render_markdown_with_complex_markdown() {
        let input = r#"# Title

Some **bold** and *italic* text.

- [ ] Task 1
- [x] Task 2

> A quote

| Col1 | Col2 |
|------|------|
| A    | B    |
"#;
        let output = render_markdown(input, "Complex");
        assert!(output.contains("<h1>Title</h1>"));
        assert!(output.contains("<strong>bold</strong>"));
        assert!(output.contains("<em>italic</em>"));
        assert!(output.contains("<table>"));
        assert!(output.contains("<blockquote>"));
    }

    #[test]
    fn test_css_styles_contains_dark_mode() {
        assert!(CSS_STYLES.contains("prefers-color-scheme: dark"));
        assert!(CSS_STYLES.contains("#0d1117"));
    }

    #[test]
    fn test_css_styles_contains_light_mode() {
        assert!(CSS_STYLES.contains("#ffffff"));
        assert!(CSS_STYLES.contains("#24292f"));
    }
}
