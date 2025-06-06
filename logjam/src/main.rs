#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod plugin_instance;
mod tab_viewer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            inner_size: Some(eframe::egui::Vec2::new(750.0, 300.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Logjam",
        options,
        Box::new(|cc| Ok(Box::new(app::Logjam::default(&cc.egui_ctx)))),
    )
}
