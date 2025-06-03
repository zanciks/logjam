#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod plugin_instance;
mod tab_viewer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Logjam",
        options,
        Box::new(|cc| Ok(Box::new(app::Logjam::default(&cc.egui_ctx)))),
    )
}
