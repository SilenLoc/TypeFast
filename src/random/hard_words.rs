use rand::seq::IteratorRandom;
use rand::thread_rng;

pub fn random_hard_english_words(max: u32) -> String {
    let mut random_words = String::new();
    for _i in 0..max {
        if let Some(word) = random_hard_words() {
            random_words.push_str(word.as_str());
        }
    }

    random_words.trim_end().into()
}

fn random_hard_words() -> Option<String> {
    let mut rng = thread_rng();
    let subs = [
        "antidisestablishmentarianism",
        "supercalifragilisticexpialidocious",
        "pneumonoultramicroscopicsilicovolcanoconiosis",
        "floccinaucinihilipilification",
        "hippopotomonstrosesquipedaliophobia",
        "worcestershire",
        "entrepreneur",
        "conscientious",
        "unquestionably",
        "labyrinth",
    ];
    let sample = subs.iter().choose(&mut rng);

    sample.map(|s| s.to_owned().to_owned())
}
