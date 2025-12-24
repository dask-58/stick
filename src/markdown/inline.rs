use crate::markdown::escape::{escape_html, push_escaped};

pub fn parse(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Bold **text**
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(end) = find_double_marker(&chars, i + 2, '*') {
                result.push_str("<strong>");
                let inner: String = chars[i + 2..end].iter().collect();
                result.push_str(&parse(&inner));
                result.push_str("</strong>");
                i = end + 2;
                continue;
            }
        }

        // Italic *text* (not preceded by *)
        if chars[i] == '*' && (i + 1 >= len || chars[i + 1] != '*') {
            if let Some(end) = find_single_marker(&chars, i + 1, '*') {
                result.push_str("<em>");
                let inner: String = chars[i + 1..end].iter().collect();
                result.push_str(&parse(&inner));
                result.push_str("</em>");
                i = end + 1;
                continue;
            }
        }

        // Inline code `text`
        if chars[i] == '`' {
            if let Some(end) = find_single_marker(&chars, i + 1, '`') {
                result.push_str("<code>");
                let inner: String = chars[i + 1..end].iter().collect();
                result.push_str(&escape_html(&inner));
                result.push_str("</code>");
                i = end + 1;
                continue;
            }
        }

        // Link [text](url)
        if chars[i] == '[' {
            if let Some((html, consumed)) = parse_link(&chars, i) {
                result.push_str(&html);
                i += consumed;
                continue;
            }
        }

        push_escaped(&mut result, chars[i]);
        i += 1;
    }

    result
}

fn parse_link(chars: &[char], start: usize) -> Option<(String, usize)> {
    let len = chars.len();
    let mut i = start + 1;

    // Find closing ]
    while i < len && chars[i] != ']' {
        i += 1;
    }
    if i >= len {
        return None;
    }
    let text_end = i;

    // Expect (
    if i + 1 >= len || chars[i + 1] != '(' {
        return None;
    }
    i += 2;
    let url_start = i;

    // Find closing )
    while i < len && chars[i] != ')' {
        i += 1;
    }
    if i >= len {
        return None;
    }
    let url_end = i;

    let text: String = chars[start + 1..text_end].iter().collect();
    let url: String = chars[url_start..url_end].iter().collect();

    let html = format!(
        "<a href=\"{}\">{}</a>",
        escape_html(&url),
        parse(&text)
    );

    Some((html, i - start + 1))
}

/// Find closing ** marker.
fn find_double_marker(chars: &[char], start: usize, marker: char) -> Option<usize> {
    let mut i = start;
    while i + 1 < chars.len() {
        if chars[i] == marker && chars[i + 1] == marker {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Find closing single marker.
fn find_single_marker(chars: &[char], start: usize, marker: char) -> Option<usize> {
    for i in start..chars.len() {
        if chars[i] == marker {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(parse("**bold**"), "<strong>bold</strong>");
    }

    #[test]
    fn test_italic() {
        assert_eq!(parse("*italic*"), "<em>italic</em>");
    }

    #[test]
    fn test_code() {
        assert_eq!(parse("`code`"), "<code>code</code>");
    }

    #[test]
    fn test_link() {
        assert_eq!(parse("[text](url)"), "<a href=\"url\">text</a>");
    }

    #[test]
    fn test_mixed() {
        assert_eq!(
            parse("**bold** and *italic*"),
            "<strong>bold</strong> and <em>italic</em>"
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(parse("a < b"), "a &lt; b");
    }
}
