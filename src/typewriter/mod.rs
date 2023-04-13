use egui::{RichText, Ui};

use crate::{scoring::Score, settings::TFSetting};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TypeState {
    input: String,
    challenge: String,
}

pub trait Challenge {
    fn to_challenge(&self) -> String;
}

impl TypeState {
    pub fn render(&mut self, ui: &mut Ui, score: &mut Score, settings: &mut TFSetting) {
        if settings.level_changed() {
            self.challenge = settings.provide_next_string().to_challenge();
            self.input.clear();
        }

        //win condition
        if self.input.eq(&self.challenge) {
            self.challenge.clear();
            self.input.clear();
            self.challenge = settings.provide_next_string().to_challenge();
            score.won(settings)
        }

        let challenge_text = RichText::new(self.challenge.to_string()).size(45.0);
        ui.heading(challenge_text);

        let input_text = RichText::new(self.input.to_string()).size(45.0);
        ui.heading(input_text);

        ui.separator();
        ui.horizontal_top(|ui| {
            ui.text_edit_multiline(&mut self.input);
            if ui.button("new").clicked() {
                self.challenge = settings.provide_next_string().to_challenge();
                self.input.clear();
            }
        });
    }
}
