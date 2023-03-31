use rand::{
    seq::IteratorRandom,
    thread_rng,
};




//He (subject) obtained (verb) his degree (object

fn random_sub() -> Option<String> {
    let mut rng = thread_rng();
    let subs = ["He", "She", "I", "They", "We"];
    let sample = subs.iter().choose(&mut rng);

    sample.map(|s| s.to_owned().to_owned())
}

fn random_verb() -> Option<String> {
    let mut rng = thread_rng();
    let subs = ["plays", "run", "shoot",];
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
    let sample = subs.iter().choose(&mut rng);

    sample.map(|s| s.to_owned().to_owned())
}


pub fn random_sentence()-> String {
    let sub = random_sub().unwrap_or("".into());
    let verb = random_verb().unwrap_or("".into());
    let object = random_object().unwrap_or("".into());
    sub + " " + &verb + " " + &object  + "." + " "
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn random_sub_should_return_a_string() {
        assert!(random_sub().is_some(), "should be some")
    }
}
