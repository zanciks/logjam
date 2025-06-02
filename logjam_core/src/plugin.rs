use abi_stable::std_types::RString;
use eframe::egui::Ui;

pub trait LogjamPlugin {
    fn title(&self) -> RString;
    fn render(&mut self, ui: &mut Ui);
}
