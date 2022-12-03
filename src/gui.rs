use std::collections::BTreeSet;

use crate::gui::egui::Ui;
use crate::hdf;
use crate::plot::Demo;
use eframe::egui;

// use eframe::egui::containers::CollapsingHeader;
#[derive(Default)]
pub(super) struct NWBView {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    h5_path: Option<String>,
    open_windows: BTreeSet<String>,
}

impl NWBView {
    fn create_group_recurision(&mut self, group: &hdf5::Group, ui: &mut Ui, ctx: &egui::Context) {
        // println!("Group Starting {}",group.name());
        ui.collapsing(group.name(), |ui| {
            let subgroups = group.groups().unwrap();
            if !subgroups.is_empty() {
                for subgroup in subgroups {
                    // println!("{}",subgroup.name());
                    self.create_group_recurision(&subgroup, ui, ctx);
                }
            }

            let datasets = group.datasets().unwrap();
            let mut dataset_names: BTreeSet<String> = BTreeSet::default();
            if !datasets.is_empty() {
                for dataset in datasets {
                    ui.monospace(dataset.name());
                    // let mut is_open = self.open_windows.contains(&dataset.name());
                    // ui.checkbox(&mut is_open, &dataset.name());
                    // set_open(&mut self.open_windows, &dataset.name().to_string(), is_open);
                    let full_dataset_name = dataset.name().to_string();
                    // split string name by "/"
                    let split_name: Vec<&str> = full_dataset_name.split('/').collect();
                    // get the last element of the split string
                    let dataset_name = split_name.last().unwrap();
                    dataset_names.insert(dataset_name.to_string());
                }
                if dataset_names.contains("data") && dataset_names.contains("timestamps") {
                    let mut is_open = self.open_windows.contains(&group.name());
                    if ui.button("plot").clicked() {
                        println!("plotting");
                        print!("is_open: {}", is_open);
                        if !is_open {
                            is_open = true;
                        }
                    }
                    if is_open {
                        let mut test_plot = Box::new(super::plot::ContextMenus::default());
                        set_open(&mut self.open_windows, &group.name(), is_open);
                        test_plot.show(ctx, &mut is_open, group);
                    }
                }
            }
        });
    }
}

impl eframe::App for NWBView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");

            // Process file when Open button is clicked
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                    self.dropped_files.clear();

                    if let Some(picked_path) = &self.picked_path {
                        self.h5_path = Some(picked_path.to_string());
                    }
                }
            }

            if let Some(hdf_path) = &self.h5_path {
                let h5_file = hdf::read_nwb_file(hdf_path).unwrap();
                ui.horizontal(|ui| {
                    ui.label("NWB Contents");
                    self.create_group_recurision(&h5_file, ui, ctx);
                });
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Process dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped NWB files:");

                    for file in &self.dropped_files {
                        let info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        ui.label(&info);
                        // self.h5_file = hdf::read_nwb_file(&info);
                    }
                });
                self.picked_path = None;
            }

            ui.horizontal(|ui| {
                ui.label("Theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                write!(text, "\n{}", path.display()).ok();
            } else if !file.mime.is_empty() {
                write!(text, "\n{}", file.mime).ok();
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &String, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
