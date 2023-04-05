use egui::{RichText, Ui};

use crate::{scoring::Score, settings::TFSetting};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TypeState {
    input: String,
    challenge: String,
}

pub trait Challenge {
    fn to_challenge(&self) -> String;
}

impl TypeState {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        score: &mut Score,
        settings: &TFSetting,
        provider: &(impl Challenge + ?Sized),
    ) {
        //win condition
        if self.input.eq(&self.challenge) {
            self.challenge.clear();
            self.input.clear();
            self.challenge = provider.to_challenge();
            score.won(settings)
        }

        let challenge_text = RichText::new(self.challenge.to_string()).size(45.0);
        ui.heading(challenge_text);

        let input_text = RichText::new(self.input.to_string()).size(45.0);
        ui.heading(input_text);

        ui.separator();
        ui.text_edit_multiline(&mut self.input);
        if ui.button("new").clicked() {
            self.challenge = provider.to_challenge();
            self.input.clear();
        }
    }
}
