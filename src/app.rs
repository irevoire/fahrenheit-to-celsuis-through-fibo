use egui_plot::{Line, Plot, PlotPoints};

#[derive(Default)]
pub struct TemplateApp {}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let range: Vec<_> = (-100..500)
                .map(|n| n as f64)
                .flat_map(|n| [n, n + 0.25, n + 0.5, n + 0.75])
                .collect();
            let fahr: PlotPoints = range
                .iter()
                .map(|&temp| [temp, celsuis_to_fahr(temp)])
                .collect();

            let fibo: PlotPoints = fibo()
                .take_while(|(_a, b)| *b < 400.)
                .map(|(a, b)| [a, b])
                .collect();
            Plot::new("temp").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(Line::new(fahr));
                plot_ui.line(Line::new(fibo));
            });
        });
    }
}

fn celsuis_to_fahr(c: f64) -> f64 {
    // (0°C × 9/5) + 32 = 32°F
    (c * 9.0 / 5.0) + 32.0
}

fn fibo() -> impl Iterator<Item = (f64, f64)> {
    let mut a = 1;
    let mut b = 0;

    std::iter::from_fn(move || {
        let tmp = a;
        a += b;
        b = tmp;

        Some((a as f64, b as f64))
    })
}
