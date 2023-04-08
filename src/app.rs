use crate::{
    scoring::Score,
    settings::TFSetting,
    typewriter::{Challenge, TypeState},
};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TypeFastApp {
    settings: TFSetting,
    type_state: TypeState,
    score: Score,
}

impl TypeFastApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TypeFastApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            TFSetting::render_state(&self.settings, ui);
            ui.label("");
            ui.collapsing("Settings", |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.text_edit_singleline(&mut self.settings.command);
                    TFSetting::process_command(&mut self.settings);
                    ui.horizontal(|ui| {
                        TFSetting::command_helpers(&mut self.settings, ui);
                    });
                })
            });
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            self.type_state.render(
                ui,
                &mut self.score,
                &self.settings,
                self.settings.provide_next_string().as_str(),
            );
            ui.label("");
            self.score.render_scoring(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl Challenge for str {
    fn to_challenge(&self) -> String {
        self.to_string()
    }
}
