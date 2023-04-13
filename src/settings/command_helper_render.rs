use egui::Ui;
use egui_extras::{Column, TableBuilder};

use super::TFSetting;
use crate::random::ALGS;

pub fn render(st: &mut TFSetting, ui: &mut Ui) {
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .auto_shrink([true, true])
                .column(Column::exact(150.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Challenges");
                    });
                })
                .body(|mut body| {
                    for alg in ALGS {
                        body.row(25.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(st.level.id != alg.id, |ui| {
                                        if ui.button(alg.description).clicked() {
                                            st.command = "level".to_owned() + " " + alg.id + ";"
                                        }
                                    });
                                });
                            });
                        });
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("size");
            ui.add(egui::DragValue::new(&mut st.size));
            if ui.button("?").clicked() {
                st.notify_help(
                    "size corresponds to how many letters, words or sentences etc. will be shown",
                );
            };
        });
    });
}
