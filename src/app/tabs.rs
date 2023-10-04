use egui_dock::NodeIndex;
use egui_dock::{DockState, Style};

use super::Module;
use super::Services;
use super::TypeFastApp;
use crate::current;
use crate::{scoring::Score, settings::TFSetting, typewriter::TypeState};

pub struct Tabs {
    pub tree: DockState<Module>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Tabs {
    pub fn new() -> Self {
        let mut tree = DockState::new(vec![Module::Typing("Typing".to_string())]);
        tree.main_surface_mut().split_below(
            NodeIndex::root(),
            0.70,
            vec![
                Module::Settings("Settings".into()),
                Module::Score("Score".into()),
                Module::Current("Current".into()),
            ],
        );

        Self { tree }
    }
}

pub fn render(ui: &mut egui::Ui, app: &mut TypeFastApp) {
    egui_dock::DockArea::new(&mut app.tabs.tree)
        .style(Style::from_egui(ui.style().as_ref()))
        .show_inside(ui, &mut app.tab_view);
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct TabView {
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
            Module::Settings(_) => self.settings.render(&mut self.services, ui),
            Module::Score(_) => self.score.render(ui),
            Module::Current(_) => current::render(&self.settings.level, ui),
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let title = match tab {
            Module::Typing(title) => title,
            Module::Settings(title) => title,
            Module::Score(title) => title,
            Module::Current(title) => title,
        };
        egui::WidgetText::RichText(title.into())
    }
}
