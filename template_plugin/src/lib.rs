use abi_stable::std_types::RString;
use eframe::egui::Ui;
use logjam_core::{LogjamPlugin, plugin::LogjamPlugin, ui::UiWrapper};

#[derive(Default, LogjamPlugin)]
struct TemplatePlugin {
    count: u32,
}

impl LogjamPlugin for TemplatePlugin {
    fn title(&self) -> RString {
        RString::from("Template Plugin")
    }
    fn render(&mut self, ui: &mut Ui) {
        ui.label(format!("click count: {}", self.count));
        ui.label("This is a label from a dll!");
        if ui.button("dll button~").clicked() {
            self.count += 1;
        }
    }
}
