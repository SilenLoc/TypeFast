use std::ops::Div;

use egui::{plot::Plot, Ui};

use crate::settings::TFSetting;
use egui::plot::{Line, PlotPoints};

use self::wpm::WordsPerDuration;

mod wpm;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Score {
    score: u128,
    score_per_duration: WordsPerDuration,
    score_plot_state: Vec<[f64; 2]>,
}

impl Score {
    pub fn render(&mut self, ui: &mut Ui) {
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
