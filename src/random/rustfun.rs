use rand::{seq::IteratorRandom, thread_rng};

//He (subject) obtained (verb) his degree (object

pub fn random_function(max: u32) -> String {
    let mut new_string = String::new();
    let new_max = if max > 20 { max / 2 } else { max };

    for _i in 0..new_max {
        new_string.push_str(&random_rust_fun());
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

fn random_params() -> Option<String> {
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
    ];

    let mut count_rng = thread_rng();
    let count: i32 = (1..5).choose(&mut count_rng).unwrap_or(1);

    let mut result = String::new();
    for now in 0..=count {
        let sample = subs.iter().choose(&mut rng);

        let column = ":";
        let types = "String";
        let comma = if now == count { "" } else { ", " };

        let param = sample.map(|s| s.to_owned().to_owned() + column + " " + types + comma);
        result.push_str(&param.unwrap_or("".into()))
    }
    Some("(".to_string() + &result + ")")
}

pub fn random_rust_fun() -> String {
    let public = random_pub().unwrap_or("".into());
    let fun = "fn";
    let identifier = random_verb().unwrap_or("".into());
    let params = random_params().unwrap_or("".into());
    let arrow = "->";
    let return_type = "String";
    public + " " + fun + " " + &identifier + &params + " " + arrow + " " + return_type + " " + "{"
}
