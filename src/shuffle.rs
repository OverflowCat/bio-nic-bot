use crate::zh::{contains_cjk_characters, tokenize};
use rand::Rng;

fn shuffle_alphabetic(s: &str) -> String {
    let length = s.len();
    if length < 4 {
        return s.to_string();
    }
    let mut rng = rand::thread_rng();
    let mut chars = s.chars();
    let mut res = String::new();
    res.push(chars.next().unwrap());
    let mut posi = 1;
    while posi < length - 1 {
        let mut char_count_to_shuffle = match rng.gen_range(0..10) {
            x if x < 4 => 1,
            x if x < 7 => 2,
            x if x < 9 => 3,
            _ => 4,
        };
        if length - 1 - posi < char_count_to_shuffle {
            char_count_to_shuffle = length - posi;
        }
        match char_count_to_shuffle {
            0 => {
                res.push(chars.next().unwrap());
                posi += 1;
            }
            x => {
                let mut tmp = String::new();
                let mut chars: Vec<char> = chars.by_ref().take(x).collect();
                tmp.push(chars.pop().unwrap());
                for c in chars {
                    tmp.push(c);
                }
                res.push_str(&tmp);
                posi += x;
            }
        }
    }
    res.push_str(&s[posi..]);
    res
}

pub fn shuffle_paragraph(s: &str) -> String {
    if s.len() <= 2 {
        return "".to_string();
    }
    let mut result = String::new();
    let words = tokenize(s);
    for word in words {
        if !contains_cjk_characters(word) {
            result.push_str(&shuffle_alphabetic(word));
        } else {
            // TODO: handle CJK words
            result.push_str(word);
        }
    }
    result
}
