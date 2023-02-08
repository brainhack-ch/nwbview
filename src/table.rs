use eframe::egui;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait PopupTable<T: std::fmt::Display> {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn set_data(&mut self, data: Vec<T>);
    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableWindow<T> {
    name: String,
    data: Vec<T>,
}

impl<T> Default for TableWindow<T> {
    fn default() -> Self {
        Self {
            name: "Table".to_string(),
            data: vec![],
        }
    }
}

impl<T: std::fmt::Display> PopupTable<T> for TableWindow<T> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(100.0)
            .show(ctx, |ui| {
                use View as _;
                self.ui(ui);
            });
    }
}

impl<T: std::fmt::Display> View for TableWindow<T> {
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

impl<T: std::fmt::Display> TableWindow<T> {
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
                body.rows(text_height, self.data.capacity(), |row_index, mut row| {
                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });
                    row.col(|ui| {
                        let item = &self.data[row_index];
                        let item_str = format!("{item}");
                        ui.label(item_str);
                    });
                });
            });
    }
}
