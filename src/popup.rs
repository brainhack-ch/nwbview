use eframe::egui;
use eframe::egui::RichText;
use eframe::egui::Window;
use crate::display_traits::{View, Show};
// use egui_extras::{Column, Size, StripBuilder, TableBuilder};

// pub trait View {
//     fn ui(&mut self, ui: &mut egui::Ui, open: &mut bool);
// }

/// Something to view
// pub trait PopupMessage {
//     // `&'static` so we can also use it as a key to store open/close state.
//     fn title(&self) -> String;
//     fn set_title(&mut self, title: String);
//     fn set_message(&mut self, msg: String);
//     // Show windows, etc
//     // fn show(&mut self, ctx: &egui::Context, open: &mut bool);
// }

/// Shows off a popup with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PopupWindow {
    title: String,
    message: String,
    open: bool,
}

impl Default for PopupWindow {
    fn default() -> Self {
        Self {
            title: "Message".to_string(),
            message: String::new(),
            open: true,
        }
    }
}

impl PopupWindow {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_message(&mut self, msg: String) {
        self.message = msg;
    }
}

impl Show for PopupWindow {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        Window::new(&self.title)
            .open(open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| self.ui(ui));
        if !self.open {
            *open = false;
        }
    }
}

impl View for PopupWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(&self.message);
        ui.vertical_centered(|sub_ui| {
            if sub_ui.button(RichText::new("Ok")).clicked() {
                self.open = false;
            }
        });
    }
}
