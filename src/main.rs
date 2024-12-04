use pulldown_cmark::{html, Parser};

fn markdown_to_html(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn generate_blog_post(title: &str, content: &str) -> String {
    format!(
        "<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>{}</title>
</head>
<body>
    <article>{}</article>
</body>
</html>",
        title,
        markdown_to_html(content)
    )
}

// Main function to read, convert, and generate blog posts
fn main() {
    let markdown_content = "## Typographic replacements

Enable typographer option to see result.

(c) (C) (r) (R) (tm) (TM) (p) (P) +-

test.. test... test..... test?..... test!....

!!!!!! ???? ,,  -- ---

\"Smartypants, double quotes\" and 'single quotes'


## Emphasis

**This is bold text**

__This is bold text__

*This is italic text*

_This is italic text_

~~Strikethrough~~";
    let title = "Random website";
    let html_content = generate_blog_post(title, &markdown_content);

    std::fs::write("index.html", html_content).unwrap();
}
