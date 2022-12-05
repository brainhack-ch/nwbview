mod gui;
mod hdf;
mod plot;
// use crate::gui;
// use crate::hdf;
// use crate::plot;
use gui::NWBView;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "NWB View",
        options,
        Box::new(|_cc| Box::new(NWBView::default())),
    );
    // let something = hdf::read_nwb_file("data/sub-anm266951_ses-20141201_behavior+icephys+ogen.nwb");
    // match something {
    //     None => println!("Noe value"),
    //     Some(x) => println!("name={} ; #groups={} ; #datasets={}", x.name, x.sub_groups.len(), x.datasets.len()),
    // }
}
