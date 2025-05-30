use crate::plugin_instance::PluginInstance;
use eframe::egui;

pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = PluginInstance;
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&tab.title).into()
    }
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.render(&mut ui.into());
    }
    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
}
