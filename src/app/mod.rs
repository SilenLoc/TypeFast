use egui_dock::Style;
use egui_notify::Toasts;

use self::tabs::{TabView, Tabs};

mod tabs;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TypeFastApp {
    #[serde(skip)]
    tabs: Tabs,
    tab_view: TabView,
    #[serde(skip)]
    addable_modules: Vec<Module>,
}

#[derive(Default)]
pub struct Services {
    pub notifier: Toasts,
}

#[derive(PartialEq)]
pub enum Module {
    Typing(String),
    Settings(String),
    Score(String),
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
            render_top(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            render_center(ui, self);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn render_center(ui: &mut egui::Ui, app: &mut TypeFastApp) {
    egui_dock::DockArea::new(&mut app.tabs.tree)
        .style(Style::from_egui(ui.style().as_ref()))
        .show_inside(ui, &mut app.tab_view);
}

fn render_top(ui: &mut egui::Ui, app: &mut TypeFastApp) {
    ui.horizontal(|ui| {
        if ui.button("Typing").clicked()
            && app
                .tabs
                .tree
                .find_tab(&Module::Typing("Typing".into()))
                .is_none()
        {
            app.tabs
                .tree
                .push_to_first_leaf(Module::Typing("Typing".into()))
        }

        if ui.button("Settings").clicked()
            && app
                .tabs
                .tree
                .find_tab(&Module::Settings("Settings".into()))
                .is_none()
        {
            app.tabs
                .tree
                .push_to_first_leaf(Module::Settings("Settings".into()))
        }

        if ui.button("Score").clicked()
            && app
                .tabs
                .tree
                .find_tab(&Module::Score("Score".into()))
                .is_none()
        {
            app.tabs
                .tree
                .push_to_first_leaf(Module::Score("Score".into()))
        }
    });
}
