use super::{Module, TypeFastApp};

pub fn render(ui: &mut egui::Ui, app: &mut TypeFastApp) {
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
        if ui.button("Current").clicked()
            && app
                .tabs
                .tree
                .find_tab(&Module::Current("Current".into()))
                .is_none()
        {
            app.tabs
                .tree
                .push_to_first_leaf(Module::Current("Current".into()))
        }
    });
}
