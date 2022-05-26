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

pub fn escape_all_markdown_v2(text: &str) -> String {
    const ESCAPE_CHARS: &[char] = &[
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];
    let mut result = String::new();
    for c in text.chars() {
        if ESCAPE_CHARS.contains(&c) {
        } else {
            result.push(c);
        }
    }
    result
}
