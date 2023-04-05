use egui::Ui;

use crate::random::{none, Algorithm, ALGS};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TFSetting {
    pub command: String,
    #[serde(skip)]
    pub level: Algorithm,
    pub size: u32,
    last_command: String,
}

impl Default for TFSetting {
    fn default() -> Self {
        Self {
            command: Default::default(),
            level: ALGS[0],
            size: 2,
            last_command: Default::default(),
        }
    }
}

impl TFSetting {
    pub fn render_state(&self, ui: &mut Ui, _ctx: &egui::Context) {
        ui.collapsing("Current", |ui| {
            ui.horizontal_top(|ui| {
                ui.label("|-|");
                ui.label("last command");
                ui.label(self.last_command.clone());
                ui.label("|-|");
                ui.label("level");
                ui.label(self.level.description);
                ui.label("|-|");
                ui.label("size");
                ui.label(format!("{}", self.size));
                ui.label("|-|");
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
            let new_level = match ALGS.into_iter().find(|alg| command.contains(alg.id)) {
                Some(alg) => alg,
                None => Algorithm {
                    id: "None",
                    version: "0",
                    description: "this algorithm does not exist",
                    lang: "mhh",
                    out_size: &0,
                    random_function: &none,
                },
            };
            self.change_level(new_level)
        }
    }

    pub fn command_helpers(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing.x = 0.5;

        for alg in ALGS {
            if ui.button(alg.description).clicked() {
                self.command = "level".to_owned() + " " + alg.id
            }
        }

        if ui.button("run command").clicked() {
            self.command.push(';');
        }

        ui.add(egui::DragValue::new(&mut self.size));
    }

    #[allow(clippy::let_and_return)]
    pub fn provide_next_string(&self) -> String {
        let next = (self.level.random_function)(self.size);
        next
    }

    pub fn change_level(&mut self, new_level: Algorithm) {
        if !self.level.id.eq(new_level.id) {
            self.size = 2;
        }
        self.level = new_level
    }
}
