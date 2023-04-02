use egui::Ui;

use crate::random::{random_english_sentences, random_english_words, random_letters};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TFSetting {
    pub command: String,
    pub level: Level,
    pub size: u32,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum Level {
    Letters,
    EnglishWords,
    EnglishSentences,
}

impl Default for TFSetting {
    fn default() -> Self {
        Self {
            command: Default::default(),
            level: Level::Letters,
            size: 2,
        }
    }
}

impl TFSetting {
    pub fn process_command(&mut self, _ui: &mut Ui, _ctx: &egui::Context) {
        let command = self.command.clone();
        if self.command.contains(';') {
            self.command.clear();
        }

        if command.contains("level") {
            if command.contains("letters") {
                self.change_level(Level::Letters);
            }
            if command.contains("words") {
                self.change_level(Level::EnglishWords);
            }
            if command.contains("sentences") {
                self.change_level(Level::EnglishSentences);
            }
        }
    }
    pub fn command_helpers(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing.x = 0.5;
        if ui.button("level letters").clicked() {
            self.command = "level letters;".into();
        }

        if ui.button("level words").clicked() {
            self.command = "level words;".into();
        }

        if ui.button("level sentences").clicked() {
            self.command = "level sentences;".into();
        }

        ui.add(egui::DragValue::new(&mut self.size));
    }

    pub fn provide_next_string(&self) -> String {
        match self.level {
            Level::Letters => random_letters(self.size),
            Level::EnglishWords => random_english_words(self.size),
            Level::EnglishSentences => random_english_sentences(self.size),
        }
    }

    pub fn change_level(&mut self, new_level: Level) {
        if !self.level.eq(&new_level) {
            self.size = 2;
        }
        self.level = new_level
    }
}
