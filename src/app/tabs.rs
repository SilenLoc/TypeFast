use egui_dock::{NodeIndex, Tree};

use super::Module;
use super::Services;
use crate::current;
use crate::{scoring::Score, settings::TFSetting, typewriter::TypeState};

pub struct Tabs {
    pub tree: Tree<Module>,
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
                Module::Current("Current".into()),
            ],
        );

        Self { tree }
    }
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
        self.services.notifier.show(ui.ctx());
        match tab {
            Module::Typing(_) => self
                .type_state
                .render(ui, &mut self.score, &mut self.settings),
            Module::Settings(_) => self.settings.render(&mut self.services, ui),
            Module::Score(_) => self.score.render_scoring(ui),
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
