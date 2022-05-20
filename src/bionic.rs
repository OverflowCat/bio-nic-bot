fn pushbuffer(buffer: &mut Vec<char>, result: &mut String) {
    let length = buffer.len();
    let half = match length % 2 {
        0 => length / 2,
        _ => (length + 1) / 2,
    };
    result.push('*');
    let part = &buffer[..half].iter().cloned().collect::<String>();
    result.push_str(&part);
    result.push('*');
    let part = &buffer[half..].iter().cloned().collect::<String>();
    result.push_str(&part);
    buffer.clear();
}

pub fn bionify(source: &str) -> String {
    let mut result = String::new();
    let mut buffer: Vec<char> = Vec::new();
    for c in source.chars() {
        if c == '*' {
            result.push_str("\\*");
        } else if c.is_alphabetic()
        /*  && !is_cjkish_codepoint(c)  */
        {
            buffer.push(c);
        } else {
            if buffer.len() > 0 {
                pushbuffer(&mut buffer, &mut result);
            }
            result.push(c);
        }
    }
    if buffer.len() > 0 {
        pushbuffer(&mut buffer, &mut result);
    }
    println!("{}", result);
    result
}
