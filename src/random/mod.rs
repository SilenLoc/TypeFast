use self::{
    letters::random_letters_inner, rustfun::random_function, sentences::random_english_sentences,
    symbols::random_symbols, words::random_english_words,
};
use crate::random::hard_words::random_hard_english_words;

mod hard_words;
mod letters;
mod rustfun;
mod sentences;
mod symbols;
mod words;

#[derive(Clone, Copy)]
pub struct Algorithm {
    pub id: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub lang: &'static str,
    pub random_function: &'static dyn Fn(u32) -> String,
}

pub const ALGS: [Algorithm; 6] = [
    Algorithm {
        id: "letters",
        version: "0.2",
        description: "some letters :)",
        lang: "western",
        random_function: &random_letters_inner,
    },
    Algorithm {
        id: "words",
        version: "0.1",
        description: "some english words",
        lang: "eng",
        random_function: &random_english_words,
    },
    Algorithm {
        id: "hard",
        version: "0.1",
        description: "hard english words to type",
        lang: "english",
        random_function: &random_hard_english_words,
    },
    Algorithm {
        id: "sentences",
        version: "0.1",
        description: "some english sentences",
        lang: "eng",
        random_function: &random_english_sentences,
    },
    Algorithm {
        id: "symbols",
        version: "0.1",
        description: "some symbols",
        lang: "well not really human",
        random_function: &random_symbols,
    },
    Algorithm {
        id: "rust_fun",
        version: "0.1",
        description: "a rust function signutare",
        lang: "rust",
        random_function: &random_function,
    },
];

pub fn none(_max: u32) -> String {
    "this algorithm does not exist".to_owned()
}
