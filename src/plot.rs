use crate::hdf;
use crate::display_traits::{View, Show};
use eframe::egui;


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
    proportional: bool,
    title: String,
    x_data: Vec<f64>,
    y_data: Vec<f64>,
    min_value: f64,
    max_value: f64,
    n_steps: usize,
    step_size: usize,
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
            proportional: false,
            title: "".to_string(),
            x_data: vec![],
            y_data: vec![],
            min_value: 0.0,
            max_value: 0.0,
            n_steps: 0,
            step_size: 0,
        }
    }
}

impl Show for PlotWindow {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(&self.title)
            .vscroll(false)
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl View for PlotWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        // Show some statistics
        ui.label(egui::RichText::new("Statistics:"));
        ui.label(egui::RichText::new(format!("min value={:?}", self.min_value)));
        ui.label(egui::RichText::new(format!("max value={:?}", self.max_value)));

        // Plot the data
        ui.checkbox(&mut self.proportional, "Equal aspect ratio")
            .on_hover_text("Ticks are the same size on both axes.");
        ui.horizontal(|ui| {
            self.trace_plot(ui).context_menu(|_ui| {});
        });

        ui.label("Zoom in zoom out using ctrl+mouse.");
    }
}

impl PlotWindow {
    pub fn get_data_from_group(&mut self, hdf5_group: &hdf::GroupTree) {
        self.title = hdf5_group.handler.name();
        self.y_data = hdf5_group
            .handler
            .dataset("data")
            .unwrap()
            .read_raw()
            .unwrap();
        let has_timestamps: bool = hdf5_group
            .handler
            .datasets()
            .unwrap()
            .iter()
            .any(|x| x.name().ends_with("timestamps"));
        self.x_data = match has_timestamps {
            false => (0..self.y_data.len())
                .collect::<Vec<usize>>()
                .iter()
                .map(|x| *x as f64)
                .collect(),
            true => hdf5_group
                .handler
                .dataset("timestamps")
                .unwrap()
                .read_raw()
                .unwrap(),
        };

        self.min_value = *self.y_data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        self.max_value = *self.y_data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        self.n_steps = self.x_data.len() - 1;
        self.step_size = compute_step_size(self.n_steps);
    }

    fn trace_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        use egui::plot::{Line, PlotPoints};
        let line = Line::new(
            (0..=self.n_steps)
                .step_by(self.step_size)
                .map(|i| [self.x_data[i], self.y_data[i]])
                .collect::<PlotPoints>(),
        );
        let mut plot = egui::plot::Plot::new("trace_plot")
            .show_axes(self.show_axes)
            .allow_drag(self.allow_drag)
            .allow_zoom(self.allow_zoom)
            .allow_scroll(self.allow_scroll)
            .center_x_axis(self.center_x_axis)
            .center_x_axis(self.center_y_axis)
            .width(self.width)
            .height(self.height);
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        plot.show(ui, |plot_ui| plot_ui.line(line))
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
