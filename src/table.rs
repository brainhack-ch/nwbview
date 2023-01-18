/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableDemo {
    striped: bool,
    resizable: bool,
    num_rows: usize,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
}

impl Default for TableDemo {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            num_rows: 300_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
        }
    }
}

impl super::Demo for TableDemo {
    fn name(&self) -> &'static str {
        "☰ Table Demo"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for TableDemo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.striped, "Striped");
                ui.checkbox(&mut self.resizable, "Resizable columns");
            });

            ui.add(
                egui::Slider::new(&mut self.num_rows, 0..=100_000)
                    .logarithmic(true)
                    .text("Num rows"),
            );
        });

        ui.separator();

        use egui_extras::{Size, StripBuilder};
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui);
                    });
                });
            });
    }
}

impl TableDemo {
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::initial(100.0).range(40.0..=300.0).resizable(true))
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        if let Some(row_nr) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_nr, None);
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    ui.strong("Expanding content");
                });
                header.col(|ui| {
                    ui.strong("Clipped text");
                });
                header.col(|ui| {
                    ui.strong("Content");
                });
            })
            .body(|body| {
                body.rows(text_height, self.num_rows, |row_index, mut row| {
                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });
                    row.col(|ui| {
                        expanding_content(ui);
                    });
                    row.col(|ui| {
                        ui.label(long_text(row_index));
                    });
                    row.col(|ui| {
                        ui.add(egui::Label::new("Thousands of rows of even height").wrap(false));
                    });
                });
            });
    }
}

fn expanding_content(ui: &mut egui::Ui) {
    let width = ui.available_width().clamp(20.0, 200.0);
    let height = ui.available_height();
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        (1.0, ui.visuals().text_color()),
    );
}

fn long_text(row_index: usize) -> String {
    format!("Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!")
}
