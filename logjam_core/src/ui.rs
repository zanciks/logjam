use super::response::Response;
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
    pub fn label(&self, text: &str) -> Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.label(text).into()
    }
    pub fn button(&self, text: &str) -> Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.button(text).into()
    }
}
