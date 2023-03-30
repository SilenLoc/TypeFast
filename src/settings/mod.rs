use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TFSetting {
    pub command: String,
    pub settings_open: bool,
}






impl Default for TFSetting {
    fn default() -> Self {
        Self {
            command: Default::default(),
            settings_open: false,
        }
    }
}

impl TFSetting {
    pub fn process_command(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let command = self.command.clone();
        if self.command.contains(";") {
            self.command.clear();
        }

        if command.eq("") {
            ui.label(&self.command);
        }

        if command.eq("open settings;") {
            self.settings_open = true
        }

        if command.eq("close settings;") {
            self.settings_open = false
        }

        if self.settings_open {
            egui::Window::new("Draft setting").show(ctx, |ui| {
                ui.label("Hey");
                if ui.button("close").clicked() {
                    self.settings_open = false
                }
            });
        }
    }
    pub fn command_helpers(&mut self, ui: &mut Ui,){
        ui.spacing_mut().item_spacing.x = 0.5;
        if ui.button("open settings").clicked() {
            self.command = "open settings;".into();
        }
        if ui.button("close settings").clicked() {
            self.command = "close settings;".into();
        }
    }
}

