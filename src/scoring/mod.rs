use egui::Ui;

use self::wpm::WordsPerDuration;

mod wpm;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Score {
    words_per_minute: WordsPerDuration,
    char_per_minute: WordsPerDuration,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            words_per_minute: WordsPerDuration::new(5),
            char_per_minute: WordsPerDuration::new(1),
        }
    }
}

impl Score {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            self.words_per_minute.render(ui, "word");
            ui.separator();
            self.char_per_minute.render(ui, "char");
        });
    }

    pub fn started_to_type(&mut self) {
        self.words_per_minute.ready();
        self.char_per_minute.ready()
    }

    pub fn set(&mut self, len: u32) {
        self.words_per_minute.set(len);
        self.char_per_minute.set(len);
    }

    pub fn won(&mut self) {
        self.words_per_minute.won();
        self.char_per_minute.won();
    }
}
