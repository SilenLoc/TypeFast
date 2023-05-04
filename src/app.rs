use egui_notify::Toasts;

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
    #[serde(skip)]
    services: Services,
}

#[derive(Default)]
pub struct Services {
    pub notifier: Toasts,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_source("all")
                .show(ui, |ui| {
                    self.settings.set_new_theme(ctx);
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            self.type_state
                                .render(ui, &mut self.score, &mut self.settings);
                            ui.horizontal_top(|ui| {
                                self.score.render_scoring(ui);

                                self.settings.render(&mut self.services, ui, ctx);
                            });
                        });
                    });
                });

            self.services.notifier.show(ctx);
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
