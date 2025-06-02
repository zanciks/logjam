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

impl<'a> Into<&'a mut egui::Ui> for &'a mut UiWrapper {
    fn into(self) -> &'a mut egui::Ui {
        unsafe { self.ui.as_mut().unwrap() }
    }
}
