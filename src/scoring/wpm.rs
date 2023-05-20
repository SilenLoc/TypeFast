use std::ops::Div;

use web_time::{Duration, Instant};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WordsPerDuration {
    pub elapsed: Duration,
    #[serde(skip)]
    start: Time,
    state: f64,
    history: Vec<f64>,
    pub avg: f64,
}

impl Default for WordsPerDuration {
    fn default() -> Self {
        Self {
            elapsed: Duration::from_millis(0),
            start: Time::now(),
            state: 0.0,
            avg: 0.0,
            history: vec![],
        }
    }
}

impl WordsPerDuration {
    pub fn ready(&mut self) {
        self.start = Time::now();
    }

    pub fn set(&mut self, words: u128) {
        self.elapsed = self.start.0.elapsed();

        self.state = (words as f64).div(self.elapsed.as_secs_f64()) * 60.0;

        self.history.push(self.state);
        self.avg = average(&self.history);
    }
}

struct Time(Instant);

impl Default for Time {
    fn default() -> Self {
        Time::now()
    }
}

impl Time {
    pub fn now() -> Self {
        Self(Instant::now())
    }
}

fn average(history: &Vec<f64>) -> f64 {
    history.iter().sum::<f64>() / history.len() as f64
}
