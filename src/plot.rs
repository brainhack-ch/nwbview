use crate::hdf;
use eframe::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, hdf5_group: &hdf::GroupTree);
}

pub trait PopupWindow {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, hdf5_group: &hdf::GroupTree);
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PlotWindow {
    show_axes: [bool; 2],
    allow_drag: bool,
    allow_zoom: bool,
    allow_scroll: bool,
    center_x_axis: bool,
    center_y_axis: bool,
    width: f32,
    height: f32,
}

impl Default for PlotWindow {
    fn default() -> Self {
        Self {
            show_axes: [true, true],
            allow_drag: true,
            allow_zoom: true,
            allow_scroll: true,
            center_x_axis: false,
            center_y_axis: false,
            width: 800.0,
            height: 400.0,
        }
    }
}

impl PopupWindow for PlotWindow {
    fn name(&self) -> &'static str {
        "☰ Context Menus"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, hdf5_group: &hdf::GroupTree) {
        egui::Window::new(hdf5_group.handler.name())
            .vscroll(false)
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui, hdf5_group));
    }
}

impl View for PlotWindow {
    fn ui(&mut self, ui: &mut egui::Ui, hdf5_group: &hdf::GroupTree) {
        ui.separator();

        ui.label("Zoom in zoom out using ctrl+mouse.");
        ui.horizontal(|ui| {
            self.trace_plot(ui, hdf5_group).context_menu(|_ui| {});
        });
    }
}

impl PlotWindow {
    fn trace_plot(&self, ui: &mut egui::Ui, hdf5_group: &hdf::GroupTree) -> egui::Response {
        let x_data: Vec<f64> = hdf5_group
            .handler
            .dataset("timestamps")
            .unwrap()
            .read_raw()
            .unwrap();
        let y_data: Vec<f64> = hdf5_group
            .handler
            .dataset("data")
            .unwrap()
            .read_raw()
            .unwrap();
        use egui::plot::{Line, PlotPoints};
        let n = x_data.len() - 1;
        let step_size = compute_step_size(n);
        let line = Line::new(
            (0..=n)
                .step_by(step_size)
                .map(|i| [x_data[i], y_data[i]])
                .collect::<PlotPoints>(),
        );
        egui::plot::Plot::new("trace_plot")
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

/// Compute the step size for the plot
fn compute_step_size(n: usize) -> usize {
    let step_size: usize = if n > 10000 {
        let log_n = (n as f64).log(10.0).ceil().powi(3);
        log_n as usize
    } else {
        1
    };
    step_size
}
