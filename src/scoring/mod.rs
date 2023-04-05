use egui::Ui;

use crate::settings::TFSetting;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Score {
    score: u128,
}

impl Score {
    pub fn render_scoring(&mut self, ui: &mut Ui) {
        ui.heading(format!("{}", self.score));
    }

    pub fn won(&mut self, settings: &TFSetting) {
        self.score += settings.level.score(settings.size) as u128;
    }
}
