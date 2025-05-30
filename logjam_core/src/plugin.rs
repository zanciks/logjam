use super::ui::Ui;
use abi_stable::std_types::RString;
pub trait LogjamPlugin {
    fn title(&self) -> RString;
    fn render(&mut self, ui: &Ui);
}
