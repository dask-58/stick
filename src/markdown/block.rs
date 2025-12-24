use crate::markdown::escape::escape_html;
use crate::markdown::inline;

pub fn parse(input: &str) -> String {
    let mut html = String::with_capacity(input.len() * 2);
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    let mut i = 0;

    while i < len {
        let trimmed = lines[i].trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // Fenced code block
        if trimmed.starts_with("```") {
            let (block, consumed) = parse_code_block(&lines, i);
            html.push_str(&block);
            i += consumed;
            continue;
        }

        // Horizontal rule
        if is_horizontal_rule(trimmed) {
            html.push_str("<hr>\n");
            i += 1;
            continue;
        }

        // Heading
        if let Some((level, content)) = parse_heading(trimmed) {
            html.push_str(&format!(
                "<h{}>{}</h{}>\n",
                level,
                inline::parse(content),
                level
            ));
            i += 1;
            continue;
        }

        // Blockquote
        if trimmed.starts_with("> ") {
            let (block, consumed) = parse_blockquote(&lines, i);
            html.push_str(&block);
            i += consumed;
            continue;
        }

        // Table
        if is_table_start(&lines, i) {
            let (block, consumed) = parse_table(&lines, i);
            html.push_str(&block);
            i += consumed;
            continue;
        }

        // Unordered list
        if is_unordered_list_item(trimmed) {
            let (block, consumed) = parse_unordered_list(&lines, i);
            html.push_str(&block);
            i += consumed;
            continue;
        }

        // Ordered list
        if is_ordered_list_item(trimmed) {
            let (block, consumed) = parse_ordered_list(&lines, i);
            html.push_str(&block);
            i += consumed;
            continue;
        }

        // Paragraph (default)
        let (block, consumed) = parse_paragraph(&lines, i);
        html.push_str(&block);
        i += consumed;
    }

    html
}

// ============================================================================
// Headings
// ============================================================================

fn parse_heading(line: &str) -> Option<(u8, &str)> {
    if line.starts_with("#### ") {
        Some((4, &line[5..]))
    } else if line.starts_with("### ") {
        Some((3, &line[4..]))
    } else if line.starts_with("## ") {
        Some((2, &line[3..]))
    } else if line.starts_with("# ") {
        Some((1, &line[2..]))
    } else {
        None
    }
}

// ============================================================================
// Horizontal Rule
// ============================================================================

fn is_horizontal_rule(line: &str) -> bool {
    let chars: Vec<char> = line.chars().filter(|c| !c.is_whitespace()).collect();
    if chars.len() < 3 {
        return false;
    }
    let first = chars[0];
    (first == '-' || first == '*' || first == '_') && chars.iter().all(|&c| c == first)
}

// ============================================================================
// Code Blocks
// ============================================================================

fn parse_code_block(lines: &[&str], start: usize) -> (String, usize) {
    let opening = lines[start].trim();
    let lang = if opening.len() > 3 { &opening[3..] } else { "" };

    let mut i = start + 1;
    let mut code_lines = Vec::new();

    while i < lines.len() {
        if lines[i].trim().starts_with("```") {
            i += 1;
            break;
        }
        code_lines.push(lines[i]);
        i += 1;
    }

    let code = escape_html(&code_lines.join("\n"));
    let html = if lang.is_empty() {
        format!("<pre><code>{}</code></pre>\n", code)
    } else {
        format!("<pre><code class=\"language-{}\">{}</code></pre>\n", lang, code)
    };

    (html, i - start)
}

// ============================================================================
// Blockquotes
// ============================================================================

fn parse_blockquote(lines: &[&str], start: usize) -> (String, usize) {
    let mut i = start;
    let mut quote_lines = Vec::new();

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("> ") {
            quote_lines.push(&trimmed[2..]);
            i += 1;
        } else if trimmed.is_empty() {
            break;
        } else {
            break;
        }
    }

    let inner = parse(&quote_lines.join("\n"));
    let html = format!("<blockquote>\n{}</blockquote>\n", inner);

    (html, i - start)
}

// ============================================================================
// Lists
// ============================================================================

fn is_unordered_list_item(line: &str) -> bool {
    line.starts_with("- ") || line.starts_with("* ")
}

fn is_ordered_list_item(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            continue;
        }
        if c == '.' {
            return chars.peek() == Some(&' ');
        }
        return false;
    }
    false
}

fn parse_unordered_list(lines: &[&str], start: usize) -> (String, usize) {
    let mut html = String::from("<ul>\n");
    let mut i = start;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if is_unordered_list_item(trimmed) {
            html.push_str("<li>");
            html.push_str(&inline::parse(&trimmed[2..]));
            html.push_str("</li>\n");
            i += 1;
        } else if trimmed.is_empty() {
            break;
        } else {
            break;
        }
    }

    html.push_str("</ul>\n");
    (html, i - start)
}

fn parse_ordered_list(lines: &[&str], start: usize) -> (String, usize) {
    let mut html = String::from("<ol>\n");
    let mut i = start;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if is_ordered_list_item(trimmed) {
            let content = trimmed.splitn(2, ". ").nth(1).unwrap_or("");
            html.push_str("<li>");
            html.push_str(&inline::parse(content));
            html.push_str("</li>\n");
            i += 1;
        } else if trimmed.is_empty() {
            break;
        } else {
            break;
        }
    }

    html.push_str("</ol>\n");
    (html, i - start)
}

// ============================================================================
// Tables
// ============================================================================

fn is_table_start(lines: &[&str], i: usize) -> bool {
    if i + 1 >= lines.len() {
        return false;
    }
    let line = lines[i].trim();
    let next = lines[i + 1].trim();
    line.starts_with('|') && next.starts_with('|') && is_separator_row(next)
}

fn is_separator_row(line: &str) -> bool {
    let cells: Vec<&str> = line.split('|').collect();
    cells
        .iter()
        .skip(1)
        .take(cells.len().saturating_sub(2))
        .all(|cell| {
            let t = cell.trim();
            !t.is_empty() && t.chars().all(|c| c == '-' || c == ':')
        })
}

fn parse_table(lines: &[&str], start: usize) -> (String, usize) {
    let mut html = String::from("<table>\n");
    let mut i = start;

    // Header
    if i < lines.len() {
        let cells = parse_table_row(lines[i]);
        html.push_str("<thead><tr>");
        for cell in cells {
            html.push_str("<th>");
            html.push_str(&inline::parse(cell.trim()));
            html.push_str("</th>");
        }
        html.push_str("</tr></thead>\n");
        i += 1;
    }

    // Skip separator
    if i < lines.len() && is_separator_row(lines[i].trim()) {
        i += 1;
    }

    // Body
    html.push_str("<tbody>\n");
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.is_empty() || !trimmed.starts_with('|') {
            break;
        }
        let cells = parse_table_row(trimmed);
        html.push_str("<tr>");
        for cell in cells {
            html.push_str("<td>");
            html.push_str(&inline::parse(cell.trim()));
            html.push_str("</td>");
        }
        html.push_str("</tr>\n");
        i += 1;
    }
    html.push_str("</tbody>\n</table>\n");

    (html, i - start)
}

fn parse_table_row(line: &str) -> Vec<&str> {
    line.trim().trim_matches('|').split('|').collect()
}

// ============================================================================
// Paragraphs
// ============================================================================

fn parse_paragraph(lines: &[&str], start: usize) -> (String, usize) {
    let mut i = start;
    let mut para_lines = Vec::new();

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.is_empty() || is_block_start(trimmed) {
            break;
        }
        para_lines.push(trimmed);
        i += 1;
    }

    let content = inline::parse(&para_lines.join(" "));
    let html = format!("<p>{}</p>\n", content);

    (html, i - start)
}

fn is_block_start(line: &str) -> bool {
    line.starts_with("# ")
        || line.starts_with("## ")
        || line.starts_with("### ")
        || line.starts_with("#### ")
        || line.starts_with("> ")
        || line.starts_with("```")
        || is_unordered_list_item(line)
        || is_ordered_list_item(line)
        || is_horizontal_rule(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        assert_eq!(parse("# H1"), "<h1>H1</h1>\n");
        assert_eq!(parse("## H2"), "<h2>H2</h2>\n");
    }

    #[test]
    fn test_paragraph() {
        assert_eq!(parse("hello"), "<p>hello</p>\n");
    }

    #[test]
    fn test_code_block() {
        assert_eq!(parse("```\ncode\n```"), "<pre><code>code</code></pre>\n");
    }

    #[test]
    fn test_list() {
        assert_eq!(parse("- a\n- b"), "<ul>\n<li>a</li>\n<li>b</li>\n</ul>\n");
    }
}
