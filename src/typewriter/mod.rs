use egui::{RichText, Ui};

use crate::{scoring::Score, settings::TFSetting};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TypeState {
    input: String,
    challenge: String,
    #[serde(skip)]
    state: State,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
enum State {
    Started,
    Typing,
    Won,
    Reset,
    Initial,
    None,
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}

pub trait Challenge {
    fn to_challenge(&self) -> String;
}

impl TypeState {
    pub fn render(&mut self, ui: &mut Ui, score: &mut Score, settings: &mut TFSetting) {
        if settings.level_changed() {
            self.state = State::Reset
        }

        let challenge_text = RichText::new(self.challenge.to_string()).size(45.0);
        ui.horizontal_wrapped(|ui| ui.heading(challenge_text));

        let input_text = RichText::new(self.input.to_string()).size(45.0);
        ui.horizontal_wrapped(|ui| ui.heading(input_text));

        ui.add_space(100.0);

        ui.horizontal_top(|ui| {
            ui.text_edit_multiline(&mut self.input);

            if ui.button("new").clicked() {
                self.state = State::Reset
            }
        });

        match self.state {
            State::Started => {
                score.started_to_type();
                self.state = State::Typing
            }
            State::Typing => {
                //win condition
                if self.input.eq(&self.challenge) && !self.input.is_empty() {
                    self.state = State::Won;
                }
            }
            State::Won => {
                self.state = State::Reset;
                score.won(settings);
                score.set();
            }
            State::Reset => {
                self.challenge = settings.provide_next_string().to_challenge();
                self.input.clear();
                self.state = State::None
            }
            State::Initial => {
                self.challenge = settings.provide_next_string().to_challenge();
                self.state = State::None
            }
            State::None => {
                if !self.input.is_empty() {
                    self.state = State::Started
                }
            }
        }
    }
}
