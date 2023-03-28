use egui::Ui;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TypeFastApp {
    command: String,
    settings_open: bool,
}

impl Default for TypeFastApp {
    fn default() -> Self {
        Self {
            command: "".into(),
            settings_open: false,
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
              ui.text_edit_singleline(&mut self.command);
                process_command(self, ui, ctx);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x= 0.5;
                    if ui.button("open settings").clicked() {
                        self.command = "open settings;".into();
                    }
                    if ui.button("close settings").clicked() {
                      self.command = "close settings;".into();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("central");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
            ui.label("placeholder");
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn process_command(app: &mut TypeFastApp, ui: &mut Ui, ctx: &egui::Context) {
    let command = app.command.clone();
    if app.command.contains(";") {
        app.command.clear();
    }

    if command.eq("") {
        ui.label(&app.command);
    }

    if command.eq("open settings;") {
        app.settings_open = true
    }

    if command.eq("close settings;") {
        app.settings_open = false
    }

    if app.settings_open {
        egui::Window::new("Draft setting").show(ctx, |ui| {
            ui.label("Hey");
            if ui.button("close").clicked() {
                app.settings_open = false
            }
        });
    }
}
