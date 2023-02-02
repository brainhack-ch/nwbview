use std::any::Any;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::mem;
use std::path::Path;

use crate::gui::egui::Ui;
use crate::hdf;
// use crate::plot::Popup;
// use crate::popup::PopupMessage;
// use crate::table::PopupTable;
use crate::display_traits::Show;
use eframe::egui::RichText;
// use eframe::egui::{self, Window};
use eframe::egui;

trait AnyShow: Any + Show {}

#[derive(Default)]
pub(crate) struct NWBView {
    pub loaded_files: Vec<hdf::FileTree>,
    pub open_windows: HashMap<String, Box<dyn Show>>,
}

impl NWBView {
    fn add_file(&mut self, path: String) {
        let input_path = Path::new(&path);
        let actual_path = match input_path.canonicalize() {
            Err(_) => {
                println!("Could not load the file '{path}'!");
                return;
            }
            Ok(x) => x,
        };

        for i in &self.loaded_files {
            let loaded_path = Path::new(&i.file.filename()).canonicalize().unwrap();

            if loaded_path == actual_path {
                println!("The file '{path}' is already loaded!");
                return;
            }
        }
        match hdf::read_nwb_file(&path) {
            None => println!("Could not load {path}"),
            Some(i) => self.loaded_files.push(i),
        }
    }
}

impl NWBView {
    fn create_group_recursion(&mut self, group: &hdf::GroupTree, ui: &mut Ui, ctx: &egui::Context) {
        let group_name = group.handler.name();
        let group_split_name: Vec<&str> = group_name.split('/').collect();
        ui.collapsing(*group_split_name.last().unwrap(), |ui| {
            let subgroups = &group.groups;
            if !subgroups.is_empty() {
                for subgroup in subgroups {
                    self.create_group_recursion(subgroup, ui, ctx);
                }
            }

            let datasets = &group.datasets;
            let mut dataset_names: BTreeSet<String> = BTreeSet::default();
            if !datasets.is_empty() {
                for dataset in datasets {
                    let split_name: Vec<&str> = dataset.split('/').collect();
                    let dataset_name = split_name.last().unwrap();
                    let mut is_open = self.open_windows.contains_key(dataset);
                    dataset_names.insert(dataset_name.to_string());
                    ui.horizontal(|horizontal_ui| {
                        horizontal_ui.monospace(dataset_name.to_string());
                        if !is_open && horizontal_ui.button(RichText::new("☰")).clicked() {
                            is_open = true;
                        } else if is_open && horizontal_ui.button(RichText::new("❌")).clicked() {
                            is_open = false;
                        };
                    });
                    if is_open {
                        if !self.open_windows.contains_key(dataset) {
                            let ds = match group.handler.dataset(dataset_name.as_ref()) {
                                Err(e) => {
                                    self.popup(
                                        &e.to_string(),
                                        ctx,
                                        dataset,
                                        &mut is_open,
                                    );
                                    continue;
                                }
                                Ok(x) => x,
                            };
                            let ds_type = ds.dtype().unwrap();
                            let type_descriptor = ds_type.to_descriptor();

                            match type_descriptor {
                                Err(e) => {
                                    self.popup(
                                        &e.to_string(),
                                        ctx,
                                        dataset,
                                        &mut is_open,
                                    );
                                    continue;
                                },
                                Ok(descriptor) => match descriptor {
                                    hdf5::types::TypeDescriptor::Float(_) => {
                                        self.build_dataset::<f64>(&ds, dataset);
                                    }
                                    hdf5::types::TypeDescriptor::VarLenUnicode => {
                                        self.build_dataset::<hdf5::types::VarLenUnicode>(
                                            &ds,
                                            dataset,
                                        );
                                    }
                                    hdf5::types::TypeDescriptor::Integer(_) => {
                                        self.build_dataset::<i64>(&ds, dataset);
                                    }
                                    hdf5::types::TypeDescriptor::Unsigned(_) => {
                                        self.build_dataset::<u64>(&ds, dataset);
                                    }
                                    hdf5::types::TypeDescriptor::Boolean => {
                                        self.build_dataset::<bool>(&ds, dataset);
                                    }
                                    // hdf5::types::TypeDescriptor::Enum(_) => todo!(),
                                    // hdf5::types::TypeDescriptor::Compound(_) => todo!(),
                                    // hdf5::types::TypeDescriptor::FixedArray(_, _) => todo!(),
                                    // hdf5::types::TypeDescriptor::FixedAscii(_) => todo!(),
                                    // hdf5::types::TypeDescriptor::FixedUnicode(_) => todo!(),
                                    // hdf5::types::TypeDescriptor::VarLenArray(_) => todo!(),
                                    hdf5::types::TypeDescriptor::VarLenAscii => {
                                        self.build_dataset::<hdf5::types::VarLenAscii>(
                                            &ds,
                                            dataset,
                                        );
                                    }
                                    _ => {
                                        self.popup(
                                            "The dataset type is not supported yet.",
                                            ctx,
                                            dataset,
                                            &mut is_open,
                                        );
                                        continue;
                                    },
                                },
                            }
                        }
                        self.open_windows[dataset].show(ctx, &mut is_open);
                    }
                    self.check_close(is_open, dataset);
                }
                if dataset_names.contains("data") {
                    let mut is_open = self.open_windows.contains_key(&group_name);
                    if ui.button(RichText::new(" 🗠 Plot")).clicked() {
                        is_open = true;
                    }
                    if is_open {
                        if !self.open_windows.contains_key(&group_name) {
                            let mut new_plot = Box::<super::plot::PlotWindow>::default();
                            new_plot.get_data_from_group(group);
                            self.open_windows.insert(group_name, new_plot);
                        }
                        self.open_windows[&group_name].show(ctx, &mut is_open);
                    }
                    self.check_close(is_open, &group_name);
                }
            }
        });
    }

    fn build_dataset<T: hdf5::H5Type + std::fmt::Display>(
        &mut self,
        ds: &hdf5::Dataset,
        dataset: &str,
    ) {
        let mut new_ds = Box::<super::table::TableWindow<T>>::default();
        new_ds.set_name(dataset.to_owned());
        if ds.is_scalar() {
            let scalar: String = ds.read_scalar::<T>().unwrap().to_string();
            new_ds.set_scalar(scalar);
        } else {
            let x_data: Vec<T> = ds
                .read_raw::<T>()
                .unwrap()
                .iter()
                .map(|x| *x as T)
                .collect();
            new_ds.set_data(x_data);
        }
        self.open_windows.insert(dataset.to_string(), new_ds);
    }

    fn popup(
        &mut self,
        msg: &str,
        ctx: &egui::Context,
        dataset: &str,
        is_open: &mut bool,
    ) {
        let msg = format!("The dataset {:?} could not be opened because of the following error:\n{:?}", dataset, msg);
        let mut new_popup = Box::<super::popup::PopupWindow>::default();
        new_popup.set_title("Dataset error".to_string());
        new_popup.set_message(msg);
        new_popup.show(ctx, &mut is_open);
        self.open_windows.insert(dataset.to_string(), new_popup);
    }

    fn check_close(&mut self, open: bool, key: &String) {
        if !(open) {
            self.open_windows.remove(key);
        }
    }
}

impl eframe::App for NWBView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("theme_panel")
            .resizable(false)
            .min_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });

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

            egui::ScrollArea::both().show(ui, |sub_ui| {
                for loaded_file in all_loaded_files.iter_mut() {
                    sub_ui.horizontal(|horizontal_ui| {
                        if horizontal_ui.button(RichText::new("❌")).clicked() {
                            loaded_file.is_opened = false; // Mark the file as closed
                        };
                        horizontal_ui.collapsing(loaded_file.file.filename(), |header_ui| {
                            for groups in &loaded_file.tree.groups {
                                self.create_group_recursion(groups, header_ui, ctx);
                            }
                        });
                    });
                }
            });

            all_loaded_files.retain(|x| x.is_opened); // Remove closed files
            mem::swap(&mut all_loaded_files, &mut self.loaded_files);
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

fn set_open(open: &mut HashMap<String, Box<dyn Any>>, key: &String, is_open: bool, handler: Box<dyn Any>) {
    if is_open {
        if !open.contains_key(key) {
            open.insert(key.to_owned(), handler);
        }
    } else {
        open.remove(key);
    }
}
