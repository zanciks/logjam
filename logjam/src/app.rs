use crate::plugin_instance::PluginInstance;
use crate::tab_viewer::TabViewer;
use eframe::egui;

pub struct Logjam {
    plugins: egui_dock::DockState<PluginInstance>,
}

// i would like to replace eframe with epi and use the epi::App::setup() function instead of this
impl Logjam {
    pub fn default(_ctx: &egui::Context) -> Logjam {
        let tabs = PluginInstance::load_all_plugins();
        let plugins = egui_dock::DockState::new(tabs);

        Logjam { plugins } 
    }
}

impl eframe::App for Logjam {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // instead of a CentralPanel, we have the plugins shown inside of a DockArea
        egui_dock::DockArea::new(&mut self.plugins)
            .show_leaf_close_all_buttons(false)
            .show_leaf_collapse_buttons(false)
            .tab_context_menus(false)
            .show(ctx, &mut TabViewer);

        ctx.request_repaint_after_secs(1.0);
    }
}
