use egui::Ui;
use egui_extras::{Column, TableBuilder};

use crate::random::Algorithm;

pub fn render(level: &Algorithm, ui: &mut Ui) {
    ui.collapsing("Current", |ui| {
        ui.add_space(10.0);
        TableBuilder::new(ui)
            .striped(true)
            .vscroll(true)
            .column(Column::exact(150.0))
            .column(Column::exact(250.0))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Challenge");
                });
                header.col(|_ui| {});
            })
            .body(|mut body| {
                body.row(25.0, |mut row| {
                    row.col(|ui| {
                        ui.label("id");
                    });
                    row.col(|ui| {
                        ui.label(level.id);
                    });
                });

                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("version");
                    });
                    row.col(|ui| {
                        ui.label(level.version);
                    });
                });

                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("description");
                    });
                    row.col(|ui| {
                        ui.label(level.description);
                    });
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("lang");
                    });
                    row.col(|ui| {
                        ui.label(level.lang);
                    });
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("score multiplier");
                    });
                    row.col(|ui| {
                        ui.label(level.out_size.to_string());
                    });
                });
            });
    });
}
