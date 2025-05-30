use eframe::egui;

#[repr(C)]
pub struct Ui {
    ui: *mut egui::Ui,
}

impl From<&mut egui::Ui> for Ui {
    fn from(ui: &mut egui::Ui) -> Self {
        Self { ui: ui as *mut egui::Ui }
    }
}

impl Ui {
    pub fn label(&self, text: &str) -> egui::Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.label(text).into()
    }
    pub fn button(&self, text: &str) -> egui::Response {
        let ui = unsafe { self.ui.as_mut().unwrap() };
        ui.button(text).into()
    }
}
