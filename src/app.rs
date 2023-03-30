use crate::{settings::TFSetting, typewriter::{TypeState, Challenge}, random::random_letters};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TypeFastApp {
    settings: TFSetting,
    type_state: TypeState
}

impl Default for TypeFastApp {
    fn default() -> Self {
        Self {
            settings: TFSetting::default(),
            type_state: TypeState::default(),
        }
    }
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
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

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
  
        self.type_state.render(ui, random_letters(32).as_str())
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}



impl Challenge for str {
    fn into_challenge(&self) -> String {
        self.to_string()
    }
}