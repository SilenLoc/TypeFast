use rand::{seq::IteratorRandom, thread_rng};
use serde::__private::de::IdentifierDeserializer;

//He (subject) obtained (verb) his degree (object

pub fn random_function(max: u32) -> String {
    let mut new_string = String::new();
    let new_max = if max > 20 { max / 2 } else { max };

    for _i in 0..new_max {
        new_string.push_str(&random_sentence());
    }

    new_string.trim().into()
}

fn random_pub() -> Option<String> {
    let mut rng = thread_rng();
    let subs = ["pub", ""];
    let sample = subs.iter().choose(&mut rng);

    sample.map(|s| s.to_owned().to_owned())
}

fn random_verb() -> Option<String> {
    let mut rng = thread_rng();
    let subs = ["plays", "run", "shoot"];
    let sample = subs.iter().choose(&mut rng);

    sample.map(|s| s.to_owned().to_owned())
}

fn random_object() -> Option<String> {
    let mut rng = thread_rng();
    let subs = [
        "people",
        "history",
        "way",
        "art",
        "world",
        "information",
        "map",
        "two",
        "family",
        "government",
        "health",
        "system",
        "computer",
        "meat",
        "year",
        "thanks",
        "music",
        "person",
        "reading",
        "method",
        "data",
        "food",
        "understanding",
        "theory",
        "law",
        "bird",
        "literature",
        "problem",
        "software",
        "control",
        "knowledge",
        "power",
        "ability",
        "economics",
        "love",
        "internet",
        "television",
        "science",
        "library",
        "nature",
        "fact",
        "product",
        "idea",
        "temperature",
        "investment",
        "area",
        "society",
        "activity",
        "story",
        "industry",
        "media",
        "thing",
        "oven",
        "community",
        "definition",
        "safety",
        "quality",
        "development",
        "language",
        "management",
        "player",
        "variety",
        "video",
        "week",
        "security",
        "country",
        "exam",
        "movie",
        "organization",
        "equipment",
        "physics",
        "analysis",
        "policy",
        "series",
        "thought",
        "basis",
        "boyfriend",
        "direction",
        "strategy",
        "technology",
        "army",
        "camera",
        "freedom",
        "paper",
        "environment",
        "child",
        "instance",
        "month",
        "truth",
        "marketing",
        "university",
        "writing",
        "article",
        "department",
        "difference",
        "goal",
        "news",
        "audience",
        "fishing",
        "growth",
        "income",
        "marriage",
        "user",
        "combination",
        "failure",
        "meaning",
        "medicine",
        "philosophy",
        "teacher",
        "communication",
        "night",
        "chemistry",
        "run",
        "shoot",
        "They",
    ];

    let mut countRng = thread_rng();
    let sample = subs.iter().choose(&mut rng);

   let column = ":";
    let types = "String";
    let comma = ",";

    let values = sample.map(|s| s.to_owned().to_owned() + column + " " + types);

 


    

    
}

pub fn random_rust_fun() -> String {
    let public = random_pub().unwrap_or("".into());
    let fun = "fn";
    let identifier = random_verb().unwrap_or("".into());
    let braces = random_object().unwrap_or("".into());
    let arrow = random_object().unwrap_or("".into());
    let return_type = random_object().unwrap_or("".into());
    public + " " + &fun + " " + &identifier  + &braces + " " + &arrow + " " + &return_type + "{"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn random_sub_should_return_a_string() {
        assert!(random_sub().is_some(), "should be some")
    }
}
