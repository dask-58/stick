mod block;
mod escape;
mod inline;

pub fn parse(input: &str) -> String {
    block::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headings() {
        assert_eq!(parse("# H1"), "<h1>H1</h1>\n");
        assert_eq!(parse("## H2"), "<h2>H2</h2>\n");
        assert_eq!(parse("### H3"), "<h3>H3</h3>\n");
        assert_eq!(parse("#### H4"), "<h4>H4</h4>\n");
    }

    #[test]
    fn test_paragraph() {
        assert_eq!(parse("Hello"), "<p>Hello</p>\n");
    }

    #[test]
    fn test_bold_italic_code() {
        assert_eq!(parse("**b**"), "<p><strong>b</strong></p>\n");
        assert_eq!(parse("*i*"), "<p><em>i</em></p>\n");
        assert_eq!(parse("`c`"), "<p><code>c</code></p>\n");
    }

    #[test]
    fn test_link() {
        assert_eq!(parse("[a](b)"), "<p><a href=\"b\">a</a></p>\n");
    }

    #[test]
    fn test_unordered_list() {
        assert_eq!(parse("- a\n- b"), "<ul>\n<li>a</li>\n<li>b</li>\n</ul>\n");
    }

    #[test]
    fn test_ordered_list() {
        assert_eq!(parse("1. a\n2. b"), "<ol>\n<li>a</li>\n<li>b</li>\n</ol>\n");
    }

    #[test]
    fn test_blockquote() {
        assert_eq!(parse("> q"), "<blockquote>\n<p>q</p>\n</blockquote>\n");
    }

    #[test]
    fn test_horizontal_rule() {
        assert_eq!(parse("---"), "<hr>\n");
    }

    #[test]
    fn test_table() {
        let input = "| a | b |\n|---|---|\n| 1 | 2 |";
        let out = parse(input);
        assert!(out.contains("<table>"));
        assert!(out.contains("<th>a</th>"));
        assert!(out.contains("<td>1</td>"));
    }

    #[test]
    fn test_code_block() {
        let input = "```\ncode\n```";
        let out = parse(input);
        assert!(out.contains("<pre><code>"));
        assert!(out.contains("code"));
    }

    #[test]
    fn test_code_block_with_lang() {
        let input = "```rust\nfn main() {}\n```";
        let out = parse(input);
        assert!(out.contains("language-rust"));
        assert!(out.contains("fn main()"));
    }
}
