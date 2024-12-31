use self::tabs::{TabView, Tabs};

mod command_center;
mod tabs;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TypeFastApp {
    #[serde(skip)]
    tabs: Tabs,
    tab_view: TabView,
    #[serde(skip)]
    addable_modules: Vec<Module>,
    command_center_open: bool,
}

#[derive(PartialEq)]
pub enum Module {
    Typing(String),
    Settings(String),
    Score(String),
    Current(String),
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
        ctx.request_repaint();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            if ui.button("Tabs").clicked() {
                self.command_center_open = true
            };
            command_center::render(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            tabs::render(ui, self);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
