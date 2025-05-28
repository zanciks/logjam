use abi_stable::std_types::RString;
use logjam_core::{plugin::LogjamPlugin, ui::UiWrapper, LogjamPlugin};

#[derive(Default, LogjamPlugin)]
struct TemplatePlugin;

impl LogjamPlugin for TemplatePlugin {
    fn title(&self) -> RString {
        RString::from("Template Plugin")
    }
    fn render(&mut self, _ui: &UiWrapper) {
        
    }
}