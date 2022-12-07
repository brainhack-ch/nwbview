use std::collections::BTreeSet;
use std::mem;
use std::path::Path;

use crate::gui::egui::Ui;
use crate::hdf;
use crate::plot::Demo;
use eframe::egui;

#[derive(Default)]
pub(crate) struct NWBView {
    pub loaded_files: Vec<hdf::FileTree>,
    pub open_windows: BTreeSet<String>,
}

impl NWBView {
    fn add_file(&mut self, path: String) {
        let input_path = Path::new(&path);
        let actual_path = match input_path.canonicalize() {
            Err(_) => {
                println!("Could not load the file '{}'!", path);
                return;
            }
            Ok(x) => x,
        };

        for i in &self.loaded_files {
            let loaded_path = Path::new(&i.file.filename()).canonicalize().unwrap();

            if loaded_path == actual_path {
                println!("The file '{}' is already loaded!", path);
                return;
            }
        }
        match hdf::read_nwb_file(&path) {
            None => println!("Could not load {}", path),
            Some(i) => self.loaded_files.push(i),
        }
    }
}

impl NWBView {
    fn create_group_recursion(&mut self, group: &hdf::GroupTree, ui: &mut Ui, ctx: &egui::Context) {
        // println!("Group Starting {}",group.name());
        ui.collapsing(group.handler.name(), |ui| {
            let subgroups = &group.groups;
            if !subgroups.is_empty() {
                for subgroup in subgroups {
                    // println!("{}",subgroup.name());
                    self.create_group_recursion(subgroup, ui, ctx);
                }
            }

            let datasets = &group.datasets;
            let mut dataset_names: BTreeSet<String> = BTreeSet::default();
            if !datasets.is_empty() {
                for dataset in datasets {
                    // let mut is_open = self.open_windows.contains(&dataset.name());
                    // ui.checkbox(&mut is_open, &dataset.name());
                    // set_open(&mut self.open_windows, &dataset.name().to_string(), is_open);
                    // let full_dataset_name = dataset.name().to_string();
                    // split string name by "/"
                    // let full_dataset_name = dataset;
                    let split_name: Vec<&str> = dataset.split('/').collect();
                    // let split_name: Vec<&str> = full_dataset_name.split('/').collect();
                    // get the last element of the split string
                    let dataset_name = split_name.last().unwrap();
                    dataset_names.insert(dataset_name.to_string());
                    ui.monospace(dataset);
                }
                if dataset_names.contains("data") && dataset_names.contains("timestamps") {
                    let mut is_open = self.open_windows.contains(&group.handler.name());
                    if ui.button("plot").clicked() {
                        println!("plotting");
                        print!("is_open: {}", is_open);
                        if !is_open {
                            is_open = true;
                        }
                    }
                    if is_open {
                        let mut test_plot = Box::new(super::plot::ContextMenus::default());
                        set_open(&mut self.open_windows, &group.handler.name(), is_open);
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
                    let picked_path = Some(path.display().to_string());

                    if let Some(x) = &picked_path {
                        self.add_file(x.to_string());
                    }
                }
            }

            let mut all_loaded_files: Vec<hdf::FileTree> = Vec::new();
            mem::swap(&mut all_loaded_files, &mut self.loaded_files);

            for loaded_file in &all_loaded_files {
                println!("The file {} is loaded", loaded_file.file.filename());
                for groups in &loaded_file.tree.groups {
                    self.create_group_recursion(groups, ui, ctx);
                }
            }

            mem::swap(&mut all_loaded_files, &mut self.loaded_files);

            // if let Some(hdf_path) = &self.h5_path {
            //     let h5_file = hdf::read_nwb_file(hdf_path).unwrap();
            //     ui.horizontal(|ui| {
            //         ui.label("NWB Contents");
            //         self.create_group_recursion(&h5_file, ui, ctx);
            //     });
            // }

            // if let Some(picked_path) = &self.picked_path {
            //     ui.horizontal(|ui| {
            //         ui.label("Picked file:");
            //         ui.monospace(picked_path);
            //     });
            // }

            // Process dropped files (if any):
            // if !self.dropped_files.is_empty() {
            //     ui.group(|ui| {
            //         ui.label("Dropped NWB files:");

            //         for file in &self.dropped_files {
            //             let info = if let Some(path) = &file.path {
            //                 path.display().to_string()
            //             } else if !file.name.is_empty() {
            //                 file.name.clone()
            //             } else {
            //                 "???".to_owned()
            //             };
            //             ui.label(&info);
            //             // self.h5_file = hdf::read_nwb_file(&info);
            //         }
            //     });
            //     self.picked_path = None;
            // }

            ui.horizontal(|ui| {
                ui.label("Theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            for file in ctx.input().raw.dropped_files.clone() {
                match file.path {
                    None => println!("Could not load the file!"),
                    Some(x) => self.add_file(x.display().to_string()),
                }
            }
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
