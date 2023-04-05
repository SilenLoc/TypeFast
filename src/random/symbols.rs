use rand::Rng;

pub fn random_symbols(max: u32) -> String {
    let mut s: String = "".into();
    for _i in 0..max {
        let letter: char = some_symbol();
        let manipulated = letter_or_space(letter);
        s.push(manipulated)
    }
    s.trim().to_owned()
}

fn letter_or_space(letter: char) -> char {
    let mut rng = rand::thread_rng();
    let lr = rng.gen_range(0..1);
    match lr {
        0 => letter,
        1 => ' ',
        _ => letter,
    }
}

fn some_symbol() -> char {
    let mut rng = rand::thread_rng();
    let lr = rng.gen_range(0..11);
    match lr {
        0 => '{',
        1 => '}',
        2 => '.',
        3 => ',',
        4 => ';',
        5 => '-',
        6 => '_',
        7 => '[',
        8 => ']',
        9 => '!',
        10 => '(',
        11 => ')',
        _ => ' ',
    }
}
