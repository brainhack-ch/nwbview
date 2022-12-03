use eframe::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Plot {
    Sin,
    Bell,
}

fn gaussian(x: f64) -> f64 {
    let var: f64 = 2.0;
    f64::exp(-(x / var).powi(2)) / (var * f64::sqrt(std::f64::consts::TAU))
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ContextMenus {
    plot: Plot,
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
            plot: Plot::Sin,
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

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .vscroll(false)
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl View for ContextMenus {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        ui.label("Right-click plot to edit it!");
        ui.horizontal(|ui| {
            self.example_plot(ui).context_menu(|_ui| {});
        });
    }
}

impl ContextMenus {
    fn example_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        use egui::plot::{Line, PlotPoints};
        let n = 128;
        let line = Line::new(
            (0..=n)
                .map(|i| {
                    use std::f64::consts::TAU;
                    let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                    match self.plot {
                        Plot::Sin => [x, x.sin()],
                        Plot::Bell => [x, 10.0 * gaussian(x)],
                    }
                })
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
