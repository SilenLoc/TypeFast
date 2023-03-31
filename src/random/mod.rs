use rand::Rng;

pub fn random_letters(max: u32) -> String {
    let mut rng = rand::thread_rng();
    let mut s: String = "".into();
    for _i in 0..max {
        let letter: char = rng.gen_range(b'A'..=b'Z') as char;
        let manipulated = big_small_space(letter);
        s.push(manipulated)
    }
    s.trim().to_string()
}

fn big_small_space(letter: char) -> char {
    let mut rng = rand::thread_rng();
    let lr = rng.gen_range(0..3);
    match lr {
        0 => letter,
        1 => {
            let chars: Vec<char> = letter.to_lowercase().to_string().chars().collect();
            chars[0]
        }
        2 => ' ',
        _ => letter,
    }
}

pub fn random_english_words(max: u32) -> String {
    let mut random_words = String::new();
    for _i in 0..max {
        random_words.push_str(random_word::gen());
        random_words.push_str(" ");
    }

    random_words.trim_end().into()
}
