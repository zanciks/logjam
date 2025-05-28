use eframe::egui;

#[repr(C)]
pub struct UiWrapper {
    ui: *mut egui::Ui,
}

impl UiWrapper {
    pub fn new(ui: &mut egui::Ui) -> Self {
        let ui = ui as *mut egui::Ui;
        UiWrapper { ui }
    }
}