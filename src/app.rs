use egui_notify::Toasts;

use crate::{
    scoring::Score,
    settings::TFSetting,
    typewriter::{Challenge, TypeState},
};

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
            ui.horizontal(|ui| {
                if ui.button("Typing").clicked()
                    && self
                        .tabs
                        .tree
                        .find_tab(&Module::Typing("Typing".into()))
                        .is_none()
                {
                    self.tabs
                        .tree
                        .push_to_first_leaf(Module::Typing("Typing".into()))
                }

                if ui.button("Settings").clicked()
                    && self
                        .tabs
                        .tree
                        .find_tab(&Module::Settings("Settings".into()))
                        .is_none()
                {
                    self.tabs
                        .tree
                        .push_to_first_leaf(Module::Settings("Settings".into()))
                }

                if ui.button("Score").clicked()
                    && self
                        .tabs
                        .tree
                        .find_tab(&Module::Score("Score".into()))
                        .is_none()
                {
                    self.tabs
                        .tree
                        .push_to_first_leaf(Module::Score("Score".into()))
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            render_ui(ui, self);
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

use egui_dock::{NodeIndex, Style, Tree};

struct Tabs {
    tree: Tree<Module>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Tabs {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![Module::Typing("Typing".to_string())]);
        tree.split_below(
            NodeIndex::root(),
            0.70,
            vec![
                Module::Settings("Settings".into()),
                Module::Score("Score".into()),
            ],
        );

        Self { tree }
    }
}

fn render_ui(ui: &mut egui::Ui, app: &mut TypeFastApp) {
    egui_dock::DockArea::new(&mut app.tabs.tree)
        .style(Style::from_egui(ui.style().as_ref()))
        .show_inside(ui, &mut app.tab_view);
}

#[derive(PartialEq)]
enum Module {
    Typing(String),
    Settings(String),
    Score(String),
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct TabView {
    settings: TFSetting,
    type_state: TypeState,
    score: Score,
    #[serde(skip)]
    services: Services,
}

impl egui_dock::TabViewer for TabView {
    type Tab = Module;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.settings.set_new_theme(ui.ctx());
        match tab {
            
            Module::Typing(_) => self
                .type_state
                .render(ui, &mut self.score, &mut self.settings),
            Module::Settings(_) => {
                
                self.settings.render(&mut self.services, ui)
            }
            Module::Score(_) => self.score.render_scoring(ui),
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let title = match tab {
            Module::Typing(title) => title,
            Module::Settings(title) => title,
            Module::Score(title) => title,
        };
        egui::WidgetText::RichText(title.into())
    }
}
