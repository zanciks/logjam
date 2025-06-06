use eframe::egui;

#[repr(C)]
pub struct UiWrapper {
    ui: *mut egui::Ui,
}

impl From<&mut egui::Ui> for UiWrapper {
    fn from(ui: &mut egui::Ui) -> Self {
        Self {
            ui: ui as *mut egui::Ui,
        }
    }
}

impl<'a> From<&'a mut UiWrapper> for &'a mut egui::Ui {
    fn from(val: &'a mut UiWrapper) -> Self {
        unsafe { val.ui.as_mut().unwrap() }
    }
}
