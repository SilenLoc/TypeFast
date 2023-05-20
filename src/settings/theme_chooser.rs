use egui::Ui;

use super::{TFSetting, TFTheme};

pub fn render_theme_choose(settings: &mut TFSetting, ui: &mut Ui) {
    egui::ComboBox::from_label("Theme")
        .selected_text(format!("{:?}", settings.theme))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut settings.theme, TFTheme::Macchiato, "Macchiato");
            ui.selectable_value(&mut settings.theme, TFTheme::Frappe, "Frappe");
            ui.selectable_value(&mut settings.theme, TFTheme::Latte, "Latte");
            ui.selectable_value(&mut settings.theme, TFTheme::Mocha, "Mocha");
        });
}
