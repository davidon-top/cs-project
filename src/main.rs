use eframe::egui;
use crate::app::App;

mod app;
mod qalculate;

fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let no = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "YAHH",
        no,
        Box::new(|cc| {
            Ok(Box::<App>::default())
        })
    ).unwrap()
}
