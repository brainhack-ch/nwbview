mod display_traits;
mod gui;
mod hdf;
mod plot;
mod popup;
mod table;
use gui::NWBView;
use image::GenericImageView;

fn main() {
    const ICON: &[u8] = include_bytes!("../static/icon.png");

    let mut options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    match image::load_from_memory(ICON) {
        Err(error) => {
            println!("Error raised while loading icon: {}", error);
            println!("Launching NWBView without the icon...");
        }
        Ok(icon) => {
            let (icon_width, icon_height) = icon.dimensions();
            options = eframe::NativeOptions {
                icon_data: Some(eframe::IconData {
                    rgba: icon.into_rgba8().to_vec(),
                    width: icon_width,
                    height: icon_height,
                }),
                drag_and_drop_support: true,
                ..Default::default()
            };
        }
    }
    eframe::run_native(
        "NWB View",
        options,
        Box::new(|_cc| Box::<NWBView>::default()),
    )
    .ok();
}
