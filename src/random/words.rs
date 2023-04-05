pub fn random_english_words(max: u32) -> String {
    let mut random_words = String::new();
    for _i in 0..max {
        random_words.push_str(random_word::gen());
        random_words.push_str(&' '.to_string());
    }

    random_words.trim_end().into()
}
