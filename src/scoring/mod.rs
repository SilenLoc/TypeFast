use std::ops::Div;

use egui::{plot::Plot, Ui};
use web_time::{Duration, Instant};

use crate::{settings::TFSetting, typewriter::Module};
use egui::plot::{Line, PlotPoints};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Score {
    score: u128,
    score_per_duration: WordsPerDuration,
    score_plot_state: Vec<[f64; 2]>,
}
impl Module for Score {
    fn discover_state(&mut self, _type_state: &mut crate::typewriter::State) {}
}

impl Score {
    pub fn render_scoring(&mut self, ui: &mut Ui) {
        ui.collapsing("Statistics", |ui| {
            ui.heading(format!("words {}", self.score));
            if ui.button("reset").clicked() {
                self.score = 0;
                self.score_plot_state.clear();
                self.score_per_duration = Default::default()
            };
            ui.vertical(|ui| {
                let score_points: PlotPoints = PlotPoints::from(self.score_plot_state.clone());

                let line = Line::new(score_points);
                Plot::new("my_plot")
                    .auto_bounds_x()
                    .auto_bounds_y()
                    .width(400.0)
                    .height(100.0)
                    .show(ui, |plot_ui| plot_ui.line(line));

                ui.label(format!(
                    "elapsed seconds {}",
                    self.score_per_duration.elapsed.as_secs_f64()
                ));
                ui.label(format!("average wpm {}", self.score_per_duration.avg));
            });
        });
    }

    pub fn started_to_type(&mut self) {
        self.score_per_duration.ready()
    }

    pub fn set(&mut self, settings: &TFSetting) {
        self.score_per_duration
            .set((settings.current_challenge_len.div(6)) as u128);
    }

    pub fn won(&mut self, settings: &TFSetting) {
        self.score += (settings.current_challenge_len.div(6)) as u128;

        self.score_plot_state.push([
            self.score_plot_state.len() as f64,
            self.score_per_duration.avg,
        ]);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WordsPerDuration {
    elapsed: Duration,
    #[serde(skip)]
    start: Time,
    state: f64,
    history: Vec<f64>,
    avg: f64,
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
