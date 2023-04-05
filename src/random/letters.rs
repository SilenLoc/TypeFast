use rand::Rng;

pub fn random_letters_inner(max: u32) -> String {
    let mut rng = rand::thread_rng();
    let mut s: String = "".into();
    for _i in 0..max {
        let letter: char = rng.gen_range(b'A'..=b'Z') as char;
        let manipulated = big_small_space(letter);
        s.push(manipulated)
    }
    s.trim().to_owned()
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
