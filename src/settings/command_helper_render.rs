use egui::Ui;
use egui_extras::{Column, TableBuilder};

use super::TFSetting;
use crate::random::ALGS;

pub fn render_commands(st: &mut TFSetting, ui: &mut Ui) {
    TableBuilder::new(ui)
        .auto_shrink([true, true])
        .column(Column::exact(100.0))
        .column(Column::exact(150.0))
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Challenge");
            });

            header.col(|ui| {
                ui.heading("Actions");
            });
        })
        .body(|mut body| {
            for alg in ALGS {
                body.row(50.0, |mut row| {
                    row.col(|ui| {
                        ui.label(alg.description);
                    });

                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("use").clicked() {
                                st.command = "level".to_owned() + " " + alg.id + ";"
                            }
                        });
                    });
                });
            }
        });
    ui.label("size");
    ui.add(egui::DragValue::new(&mut st.size));
    ui.collapsing("?", |ui| {
        ui.label("a size of 3 with the challenge 'english words' means 3 words");
    });
}
