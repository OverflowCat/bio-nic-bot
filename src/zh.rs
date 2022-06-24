use jieba_rs::Jieba;
use lazy_static::*;

lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

pub fn tokenize(text: &str) -> Vec<&str>{
    let words = JIEBA.cut(text, false);
    words
}
