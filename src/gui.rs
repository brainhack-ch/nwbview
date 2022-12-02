use crate::hdf;
use eframe::egui;

use eframe::egui::containers::CollapsingHeader;
use egui::*;

// #[derive(Default)]
pub(super) struct NWBView {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    h5_file: Option<hdf5::File>,
    tree: Tree,
}

impl Default for NWBView {
    fn default() -> Self {
        NWBView {
            dropped_files: Default::default(),
            picked_path: None,
            h5_file: None,
            tree: Tree::demo(),
        }
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

                    if let Some(picked_path) = &self.picked_path {
                        self.h5_file = hdf::read_nwb_file(picked_path);
                    }
                }
            }

            if let Some(hdf_file) = &self.h5_file {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    let groups = hdf::get_subgroups(hdf_file);

                    ui.collapsing("groups", |ui| {
                        ui.collapsing("subgroups", |ui| {
                            for group in groups {
                                ui.monospace(group.name());
                                hdf::get_subgroups(&group);
                            }
                        });
                    });
                    CollapsingHeader::new("Tree")
                        .default_open(false)
                        .show(ui, |ui| self.tree.ui(ui));
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
                        self.h5_file = hdf::read_nwb_file(&info);
                    }
                });
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
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

#[derive(Clone, Copy, PartialEq)]
enum Action {
    Keep,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct Tree(Vec<Tree>);

impl Tree {
    pub fn demo() -> Self {
        Self(vec![
            Tree(vec![Tree::default(); 4]),
            Tree(vec![Tree(vec![Tree::default(); 2]); 3]),
        ])
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Action {
        self.ui_impl(ui, 0, "root")
    }
}

impl Tree {
    fn ui_impl(&mut self, ui: &mut Ui, depth: usize, name: &str) -> Action {
        CollapsingHeader::new(name)
            .default_open(depth < 1)
            .show(ui, |ui| self.children_ui(ui, depth))
            .body_returned
            .unwrap_or(Action::Keep)
    }

    fn children_ui(&mut self, ui: &mut Ui, depth: usize) -> Action {
        self.0 = std::mem::take(self)
            .0
            .into_iter()
            .enumerate()
            .filter_map(|(i, mut tree)| {
                if tree.ui_impl(ui, depth + 1, &format!("child #{}", i)) == Action::Keep {
                    Some(tree)
                } else {
                    None
                }
            })
            .collect();

        Action::Keep
    }
}
