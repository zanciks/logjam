mod app;
mod plugin_instance;
mod tab_viewer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Logjam",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.25);
            Ok(Box::<app::Logjam>::default())
        }),
    )
}
