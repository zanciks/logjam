use abi_stable::std_types::RString;
use logjam_core::{LogjamPlugin, plugin::LogjamPlugin, ui::UiWrapper};

#[derive(Default, LogjamPlugin)]
struct TemplatePlugin;

impl LogjamPlugin for TemplatePlugin {
    fn title(&self) -> RString {
        RString::from("Template Plugin")
    }
    fn render(&mut self, ui: &UiWrapper) {
        ui.label("This is a label from a dll!");
        if ui.button("dll button~").clicked() {
            ui.label("one frame test label");
        }
    }
}
