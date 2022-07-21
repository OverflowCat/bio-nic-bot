use jieba_rs::Jieba;
use lazy_static::*;

lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

pub fn contains_cjk_characters(s: &str) -> bool {
    s.chars().any(|c| c >= '\u{4E00}' && c <= '\u{9FFF}')
}

pub fn tokenize(text: &str) -> Vec<&str>{
    let words = JIEBA.cut(text, false);
    words
}

pub fn zh_possibility(text: &str) -> f32 {
    let chars = text.chars();
    let mut total = 0u32;
    let mut zh_count = 0u32;
    for c in chars {
        match c {
            '\u{4E00}'..='\u{9FA5}' => zh_count += 1,
            _ => (),
        };
        total += 1;
    }
    zh_count as f32 / total as f32
}
