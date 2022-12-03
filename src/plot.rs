use eframe::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, hdf5_group: &hdf5::Group);
}

pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, hdf5_group: &hdf5::Group);
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ContextMenus {
    show_axes: [bool; 2],
    allow_drag: bool,
    allow_zoom: bool,
    allow_scroll: bool,
    center_x_axis: bool,
    center_y_axis: bool,
    width: f32,
    height: f32,
}

impl Default for ContextMenus {
    fn default() -> Self {
        Self {
            show_axes: [true, true],
            allow_drag: true,
            allow_zoom: true,
            allow_scroll: true,
            center_x_axis: false,
            center_y_axis: false,
            width: 400.0,
            height: 200.0,
        }
    }
}

impl Demo for ContextMenus {
    fn name(&self) -> &'static str {
        "☰ Context Menus"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, hdf5_group: &hdf5::Group) {
        egui::Window::new(self.name())
            .vscroll(false)
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui, hdf5_group));
    }
}

impl View for ContextMenus {
    fn ui(&mut self, ui: &mut egui::Ui, hdf5_group: &hdf5::Group) {
        ui.separator();

        ui.label("Right-click plot to edit it!");
        ui.horizontal(|ui| {
            self.example_plot(ui, hdf5_group).context_menu(|_ui| {});
        });
    }
}

impl ContextMenus {
    fn example_plot(&self, ui: &mut egui::Ui, hdf5_group: &hdf5::Group) -> egui::Response {
        let x_data: Vec<f64> = hdf5_group
            .dataset("timestamps")
            .unwrap()
            .read_raw()
            .unwrap();
        let y_data: Vec<f64> = hdf5_group.dataset("data").unwrap().read_raw().unwrap();
        use egui::plot::{Line, PlotPoints};
        let n = x_data.len() - 1;
        let line = Line::new(
            (0..=n)
                .map(|i| [x_data[i], y_data[i]])
                .collect::<PlotPoints>(),
        );
        egui::plot::Plot::new("example_plot")
            .show_axes(self.show_axes)
            .allow_drag(self.allow_drag)
            .allow_zoom(self.allow_zoom)
            .allow_scroll(self.allow_scroll)
            .center_x_axis(self.center_x_axis)
            .center_x_axis(self.center_y_axis)
            .width(self.width)
            .height(self.height)
            .data_aspect(1.0)
            .show(ui, |plot_ui| plot_ui.line(line))
            .response
    }
}
