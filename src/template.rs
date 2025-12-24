const TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <style>
        :root {
            --bg: #fafafa;
            --fg: #1a1a1a;
            --muted: #666;
            --border: #e0e0e0;
            --code-bg: #f5f5f5;
            --max-width: 42rem;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.6;
            color: var(--fg);
            background: var(--bg);
            padding: 2rem 1rem;
        }
        main { max-width: var(--max-width); margin: 0 auto; }
        h1 { font-size: 2rem; margin-bottom: 1rem; }
        h2 { font-size: 1.5rem; margin: 2rem 0 0.75rem; }
        h3 { font-size: 1.25rem; margin: 1.5rem 0 0.5rem; }
        h4 { font-size: 1rem; margin: 1.25rem 0 0.5rem; color: var(--muted); }
        p { margin-bottom: 1rem; }
        a { color: #0066cc; text-decoration: none; }
        a:hover { text-decoration: underline; }
        code {
            font-family: "SF Mono", Menlo, Consolas, monospace;
            font-size: 0.875em;
            background: var(--code-bg);
            padding: 0.2em 0.4em;
            border-radius: 4px;
        }
        pre {
            background: #282c34;
            color: #abb2bf;
            padding: 1rem;
            border-radius: 6px;
            overflow-x: auto;
            margin: 1rem 0;
        }
        pre code {
            background: none;
            padding: 0;
            font-size: 0.85rem;
            color: inherit;
        }
        strong { font-weight: 600; }
        em { font-style: italic; }
        ul, ol { margin: 0 0 1rem 1.5rem; }
        li { margin-bottom: 0.25rem; }
        blockquote {
            border-left: 3px solid var(--border);
            padding-left: 1rem;
            margin: 1rem 0;
            color: var(--muted);
        }
        hr { border: none; border-top: 1px solid var(--border); margin: 2rem 0; }
        table { border-collapse: collapse; margin: 1rem 0; width: 100%; }
        th, td { border: 1px solid var(--border); padding: 0.5rem; text-align: left; }
        th { background: var(--code-bg); font-weight: 600; }
    </style>
</head>
<body>
    <main>
{{content}}
    </main>
</body>
</html>
"#;

pub fn render(title: &str, content: &str) -> String {
    TEMPLATE
        .replace("{{title}}", title)
        .replace("{{content}}", content)
}
