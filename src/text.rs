pub fn escape_markdown_v2(text: &str) -> String {
    const ESCAPE_CHARS: &[char] = &[
        '>', '#', '+', '-', '|', '{', '}', '.', '!', '[', ']', '(', ')',
    ];
    let mut result = String::new();
    for c in text.chars() {
        if ESCAPE_CHARS.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}
