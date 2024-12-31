use random_word::Lang;

pub fn random_english_words(max: u32) -> String {
    let mut random_words = String::new();
    for _i in 0..max {
        random_words.push_str(random_word::gen(Lang::En));
        random_words.push(' ');
    }

    random_words.trim_end().into()
}
