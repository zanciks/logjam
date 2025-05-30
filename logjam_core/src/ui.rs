use eframe::egui;

#[repr(C)]
pub struct Ui {
    ui: *mut egui::Ui,
}

impl Ui {
    pub fn new(ui: &mut egui::Ui) -> Self {
        let ui = ui as *mut egui::Ui;
        Ui { ui }
    }
    pub fn label(&self, text: &str) -> egui::Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.label(text).into()
    }
    pub fn button(&self, text: &str) -> egui::Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.button(text).into()
    }
}
