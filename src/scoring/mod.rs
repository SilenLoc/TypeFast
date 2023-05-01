use std::{
    ops::Div,
    time::{Duration, Instant},
};

use egui::{plot::Plot, Ui};

use crate::settings::TFSetting;
use egui::plot::{Line, PlotPoints};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Score {
    score: u128,
    score_per_duration: ScorePerDuration,
    score_plot_state: Vec<[f64; 2]>,
}

impl Score {
    pub fn render_scoring(&mut self, ui: &mut Ui) {
        ui.heading(format!("{}", self.score));
        ui.horizontal_top(|ui| {
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
            ui.label(format!(
                "average score per second {}",
                self.score_per_duration.avg
            ));
        });
    }

    pub fn started_to_type(&mut self) {
        self.score_per_duration.ready()
    }

    pub fn set(&mut self) {
        self.score_per_duration.set(self.score);
    }

    pub fn won(&mut self, settings: &TFSetting) {
        self.score += (settings.level.out_size * settings.size) as u128;

        self.score_plot_state.push([
            self.score_plot_state.len() as f64,
            self.score_per_duration.avg,
        ]);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScorePerDuration {
    elapsed: Duration,
    #[serde(skip)]
    start: Time,
    state: f64,
    history: Vec<f64>,
    avg: f64,
}

impl Default for ScorePerDuration {
    fn default() -> Self {
        Self {
            elapsed: Duration::from_millis(0),
            start: Time(Instant::now()),
            state: 0.0,
            avg: 0.0,
            history: vec![],
        }
    }
}

impl ScorePerDuration {
    pub fn ready(&mut self) {
        self.start = Time::now();
    }

    pub fn set(&mut self, score: u128) {
        self.elapsed = self.start.0.elapsed();

        self.state = (score as f64).div(self.elapsed.as_secs_f64());

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
