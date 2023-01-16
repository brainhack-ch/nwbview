mod gui;
mod hdf;
mod plot;
use gui::NWBView;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "NWB View",
        options,
        Box::new(|_cc| Box::<NWBView>::default()),
    );
}
