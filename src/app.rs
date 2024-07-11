use egui_plot::{Line, Plot};

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
            let fibo: Vec<_> = fibo()
                .take_while(|(_a, b)| *b < 1000.)
                .map(|(a, b)| [a, b])
                .collect();
            let fahr: Vec<_> = fibo
                .iter()
                .map(|[celsius, _]| [*celsius, celsuis_to_fahr(*celsius)])
                .collect();
            let diff: Vec<_> = fibo
                .iter()
                .zip(fahr.iter())
                .map(|([x, fibo], [_, fahr])| [*x, *fahr - *fibo])
                .collect();

            Plot::new("temp").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(Line::new(fahr).name("Celsius to Fahrenheit"));
                plot_ui.line(Line::new(fibo).name("Fibonacci"));
                plot_ui.line(Line::new(diff).name("Fahr - Fibo"));
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
