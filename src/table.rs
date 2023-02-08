use crate::display_traits::{Show, View};
use eframe::egui;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableWindow<T> {
    name: String,
    data: Option<Vec<T>>,
    scalar: Option<String>,
}

impl<T: hdf5::H5Type + std::fmt::Display> Default for TableWindow<T> {
    fn default() -> Self {
        Self {
            name: "Table".to_string(),
            data: None,
            scalar: None,
        }
    }
}

impl<T: hdf5::H5Type + std::fmt::Display> TableWindow<T> {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = Some(data);
    }

    pub fn set_scalar(&mut self, scalar: String) {
        self.scalar = Some(scalar);
    }
}

impl<T: hdf5::H5Type + std::fmt::Display> Show for TableWindow<T> {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        if self.data.is_some() {
            egui::Window::new(&self.name)
                .open(open)
                .resizable(true)
                .default_width(100.0)
                .show(ctx, |ui| {
                    use View as _;
                    self.ui(ui);
                });
        } else if self.scalar.is_some() {
            egui::Window::new(&self.name)
                .open(open)
                .vscroll(true)
                .resizable(true)
                .default_height(300.0)
                .show(ctx, |ui| {
                    ui.label(format!("{:?}", self.scalar.as_ref().unwrap()));
                });
        } else {
            // Nothing to show
            // Should raise something here?
        }
    }
}

impl<T: hdf5::H5Type + std::fmt::Display> View for TableWindow<T> {
    fn ui(&mut self, ui: &mut egui::Ui) {
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(50.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui);
                    });
                });
            });
    }
}

impl<T: hdf5::H5Type + std::fmt::Display> TableWindow<T> {
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

        let table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(
                Column::initial(40.0)
                    .at_least(20.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(
                Column::initial(150.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            );

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Index");
                });
                header.col(|ui| {
                    ui.strong("Values");
                });
            })
            .body(|body| {
                body.rows(
                    text_height,
                    self.data.as_ref().unwrap().capacity(),
                    |row_index, mut row| {
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            let item = &self.data.as_ref().unwrap()[row_index];
                            let item_str = format!("{item}");
                            ui.label(item_str);
                        });
                    },
                );
            });
    }
}
