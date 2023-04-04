use egui::Ui;

use crate::random::{random_english_sentences, random_english_words, random_letters};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TFSetting {
    pub command: String,
    pub level: Level,
    pub size: u32,
    last_command: String,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum Level {
    Letters,
    EnglishWords,
    EnglishSentences,
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Letters => "Letters",
            Level::EnglishWords => "English words",
            Level::EnglishSentences => "English sentences",
        }
    }
}

impl Default for TFSetting {
    fn default() -> Self {
        Self {
            command: Default::default(),
            level: Level::Letters,
            size: 2,
            last_command: Default::default(),
        }
    }
}

impl TFSetting {
    pub fn render_state(&self, ui: &mut Ui, _ctx: &egui::Context) {
        ui.separator();
        ui.collapsing("Current", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("|-|");
                ui.label("last command");
                ui.label(self.last_command.clone());
                ui.label("|-|");
                ui.label("level");
                ui.label(self.level.as_str());
                ui.label("|-|");
                ui.label("size");
                ui.label(format!("{}", self.size));
            });
        });
    }

    pub fn process_command(&mut self, _ui: &mut Ui, _ctx: &egui::Context) {
        let command = self.command.clone();
        if self.command.contains(';') {
            self.last_command = command.clone();
            self.command.clear();
        }

        if command.contains("level") {
            if command.contains("letters;") {
                self.change_level(Level::Letters);
            }
            if command.contains("words;") {
                self.change_level(Level::EnglishWords);
            }
            if command.contains("sentences;") {
                self.change_level(Level::EnglishSentences);
            }
        }
    }
    pub fn command_helpers(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing.x = 0.5;
        if ui.button("level letters").clicked() {
            self.command = "level letters".into();
        }

        if ui.button("level words").clicked() {
            self.command = "level words".into();
        }

        if ui.button("level sentences").clicked() {
            self.command = "level sentences".into();
        }

        if ui.button("run command").clicked() {
            self.command.push(';');
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
