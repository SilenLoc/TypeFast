use crate::{
    settings::TFSetting,
    typewriter::{Challenge, TypeState},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TypeFastApp {
    settings: TFSetting,
    type_state: TypeState,
}

impl TypeFastApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TypeFastApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel_0").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.text_edit_singleline(&mut self.settings.command);

                TFSetting::process_command(&mut self.settings, ui, ctx);

                ui.horizontal(|ui| {
                    TFSetting::command_helpers(&mut self.settings, ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.type_state
                .render(ui, self.settings.provide_next_string().as_str())
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
