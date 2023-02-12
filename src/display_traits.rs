use eframe::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Show {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
