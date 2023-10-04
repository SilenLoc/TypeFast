use std::ops::Div;

use egui::Ui;
use egui_plot::{Line, PlotPoints};
use web_time::{Duration, Instant};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WordsPerDuration {
    pub elapsed: Duration,
    #[serde(skip)]
    start: Time,
    state: f64,
    history: Vec<f64>,
    pub avg: f64,
    score_plot_state: Vec<[f64; 2]>,
    wpm_factor: u32,
    wpm_factor_default: u32,

    show_axes: bool,
    reset_plot: bool,
}

impl WordsPerDuration {
    pub fn render(&mut self, ui: &mut Ui, title: &str) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.horizontal_top(|ui| {
                    ui.heading(format!(
                        "average {}pm {}",
                        title.chars().next().unwrap_or('x'),
                        self.avg
                    ));

                    ui.add(
                        egui::DragValue::new(&mut self.wpm_factor)
                            .clamp_range(1..=10)
                            .suffix(format!(" chars per {}", title)),
                    );

                    if ui.button("reset").clicked() {
                        self.elapsed = Duration::ZERO;
                        self.state = 0.0;
                        self.start = Time::now();
                        self.wpm_factor = self.wpm_factor_default;
                        self.avg = 0.0;
                        self.history.clear();
                        self.score_plot_state.clear();
                    };
                });
            });

            let score_points: PlotPoints = PlotPoints::from(self.score_plot_state.clone());

            let line = Line::new(score_points);
            ui.group(|ui| {
                let plot = egui_plot::Plot::new(title)
                    .auto_bounds_x()
                    .auto_bounds_y()
                    .width(400.0)
                    .height(100.0)
                    .show_axes([self.show_axes; 2])
                    .include_x(0.0)
                    .include_y(0.0);

                if self.reset_plot {
                    plot.reset().show(ui, |plot_ui| plot_ui.line(line));
                } else {
                    plot.show(ui, |plot_ui| plot_ui.line(line));
                }

                ui.vertical(|ui| {
                    self.reset_plot = ui.checkbox(&mut self.show_axes, "show axes").changed();
                });
            });
        });
    }

    pub fn new(wpm_factor_default: u32) -> Self {
        Self {
            elapsed: Duration::from_millis(0),
            start: Time::now(),
            state: 0.0,
            avg: 0.0,
            history: vec![],
            score_plot_state: vec![],
            show_axes: false,
            reset_plot: false,
            wpm_factor: wpm_factor_default,
            wpm_factor_default,
        }
    }

    pub fn ready(&mut self) {
        self.start = Time::now();
    }

    pub fn set(&mut self, len: u32) {
        self.elapsed = self.start.0.elapsed();

        self.state =
            ((len as f64).div(self.elapsed.as_secs_f64()) * 60.0).div(self.wpm_factor as f64);

        self.history.push(self.state);
        self.avg = average(&self.history);
    }

    pub fn won(&mut self) {
        self.score_plot_state
            .push([self.score_plot_state.len() as f64, self.avg]);
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
