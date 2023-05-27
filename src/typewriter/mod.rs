use egui::{RichText, Ui};

use crate::{scoring::Score, settings::TFSetting};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TypeState {
    input: String,
    #[serde(skip)]
    state: State,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum State {
    Started(String),
    Typing(String),
    Won(String),
    Reset,
    Initial,
    None(String),
}

impl Default for State {
    fn default() -> Self {
        Self::Initial
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

        match self.state.clone() {
            State::Started(chal) => {
                self.inner_render(ui, chal);
            }
            State::Typing(chal) => {
                self.inner_render(ui, chal);
            }
            State::Won(chal) => {
                self.inner_render(ui, chal);
            }
            State::Reset => self.inner_render(ui, "".to_owned()),
            State::Initial => self.inner_render(ui, "".to_owned()),
            State::None(chal) => {
                self.inner_render(ui, chal);
            }
        }

        match self.state.clone() {
            State::Started(chal) => {
                score.started_to_type();
                self.state = State::Typing(chal)
            }
            State::Typing(chal) => {
                //win condition
                if self.input.eq(&chal) && !self.input.is_empty() {
                    self.state = State::Won(chal);
                }
            }
            State::Won(chal) => {
                settings.current_challenge_len = chal.len() as u32;
                self.state = State::Reset;
                score.won();
                score.set(settings.current_challenge_len);
            }
            State::Reset => {
                self.state = State::None(settings.provide_next_string().to_challenge());
                self.input.clear();
            }
            State::Initial => {
                self.state = State::None(settings.provide_next_string().to_challenge());
            }
            State::None(chal) => {
                if !self.input.is_empty() {
                    self.state = State::Started(chal)
                }
            }
        }
    }

    fn inner_render(&mut self, ui: &mut Ui, chal: String) {
        let challenge_text = RichText::new(chal).size(30.0);
        ui.horizontal_wrapped(|ui| ui.heading(challenge_text));

        let input_text = RichText::new(self.input.to_string()).size(30.0);
        ui.horizontal_wrapped(|ui| ui.heading(input_text));

        ui.add_space(100.0);

        ui.horizontal_top(|ui| {
            ui.text_edit_multiline(&mut self.input);

            if ui.button("new").clicked() {
                self.state = State::Reset
            }
        });
    }
}

impl Challenge for str {
    fn to_challenge(&self) -> String {
        self.to_string()
    }
}
