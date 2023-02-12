mod display_traits;
mod gui;
mod hdf;
mod plot;
mod popup;
mod table;
use gui::NWBView;

fn main() {
    let icon = image::open("static/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "NWB View",
        options,
        Box::new(|_cc| Box::<NWBView>::default()),
    )
    .ok();
}
