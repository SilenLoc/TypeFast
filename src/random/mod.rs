use self::{
    letters::random_letters_inner, sentences::random_english_sentences, symbols::random_symbols,
    words::random_english_words,
};

mod letters;
mod sentences;
mod symbols;
mod words;

#[derive(Clone, Copy)]
pub struct Algorithm {
    pub id: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub lang: &'static str,
    pub out_size: &'static u32,
    pub random_function: &'static dyn Fn(u32) -> String,
}

pub const ALGS: [Algorithm; 4] = [
    Algorithm {
        id: "letters",
        version: "0.1",
        description: "some letters :)",
        lang: "human",
        out_size: &1,
        random_function: &random_letters_inner,
    },
    Algorithm {
        id: "words",
        version: "0.1",
        description: "some english words",
        lang: "eng",
        out_size: &2,
        random_function: &random_english_words,
    },
    Algorithm {
        id: "sentences",
        version: "0.1",
        description: "some english sentences",
        lang: "eng",
        out_size: &3,
        random_function: &random_english_sentences,
    },
    Algorithm {
        id: "symbols",
        version: "0.1",
        description: "some symbols",
        lang: "well not really human",
        out_size: &2,
        random_function: &random_symbols,
    },
];

pub fn none(_max: u32) -> String {
    "this algorithm does not exist".to_owned()
}
