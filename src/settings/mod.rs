mod command_helper_render;
mod command_processer;
mod theme_chooser;

use egui::Context;

use crate::{
    app::Services,
    random::{Algorithm, ALGS},
};

use self::command_processer::process_command;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TFSetting {
    pub command: String,
    last_command: String,
    last_theme: TFTheme,
    pub theme: TFTheme,
    #[serde(skip)]
    pub level: Algorithm,
    pub size: u32,
    level_changed: bool,
    pub current_challenge_len: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone, Copy)]

pub enum TFTheme {
    Macchiato,
    Frappe,
    Latte,
    Mocha,
    Default,
}

impl TFTheme {
    pub fn to_cat_theme(self) -> catppuccin_egui::Theme {
        match self {
            TFTheme::Macchiato => catppuccin_egui::MACCHIATO,
            TFTheme::Frappe => catppuccin_egui::FRAPPE,
            TFTheme::Latte => catppuccin_egui::LATTE,
            TFTheme::Mocha => catppuccin_egui::MOCHA,
            TFTheme::Default => catppuccin_egui::MACCHIATO,
        }
    }
}

impl Default for TFSetting {
    fn default() -> Self {
        Self {
            command: Default::default(),
            last_command: Default::default(),
            level: ALGS[1],
            size: 1,
            level_changed: false,
            last_theme: TFTheme::Default,
            theme: TFTheme::Macchiato,
            current_challenge_len: 0,
        }
    }
}

impl TFSetting {
    pub fn render(&mut self, services: &mut Services, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            process_command(self, services);
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                command_helper_render::render(self, ui);
                ui.add_space(10.0);
                theme_chooser::render(self, ui);
            });
        });
    }

    pub fn level_changed(&mut self) -> bool {
        let old = self.level_changed;
        self.level_changed = false;
        old
    }

    pub fn set_new_theme(&mut self, ctx: &Context) {
        if self.last_theme != self.theme {
            catppuccin_egui::set_theme(ctx, self.theme.to_cat_theme());
            self.last_theme = self.theme
        }
    }

    pub fn notify_help(&mut self, text: &str) {
        self.command = "help".to_string() + " " + text + ";";
    }

    #[allow(clippy::let_and_return)]
    pub fn provide_next_string(&self) -> String {
        let next = (self.level.random_function)(self.size);
        next
    }

    pub fn change_level(&mut self, new_level: Algorithm) {
        if !self.level.id.eq(new_level.id) {
            self.level_changed = true;
            self.size = 1;
            self.level = new_level
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::random::none;

    use super::*;

    fn create_alg_1() -> Algorithm {
        Algorithm {
            id: "one",
            version: "",
            description: "",
            lang: "",
            random_function: &none,
        }
    }

    fn create_alg_2() -> Algorithm {
        Algorithm {
            id: "two",
            version: "",
            description: "",
            lang: "",
            random_function: &none,
        }
    }

    #[test]
    fn should_change_level_and_reset_size_if_different_algorithm() {
        let mut settings = TFSetting {
            size: 500,
            ..Default::default()
        };

        let alg1 = create_alg_1();
        settings.change_level(alg1);
        assert_eq!(settings.size, 1);

        settings.size = 400;

        let alg2 = create_alg_2();
        settings.change_level(alg2);
        assert_eq!(settings.size, 1);
    }

    #[test]
    fn should_change_level_and_not_reset_size_if_same_algorithm() {
        let mut settings = TFSetting {
            size: 500,
            level: create_alg_1(),
            ..Default::default()
        };

        let alg1 = create_alg_1();
        settings.change_level(alg1);
        assert_eq!(settings.size, 500);

        settings.size = 400;

        let alg2 = create_alg_1();
        settings.change_level(alg2);
        assert_eq!(settings.size, 400);
    }
}
