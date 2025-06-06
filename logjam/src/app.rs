use crate::plugin_instance::PluginInstance;
use crate::tab_viewer::TabViewer;
use eframe::egui;
use toml_edit::DocumentMut;

pub struct Logjam {
    plugins: egui_dock::DockState<PluginInstance>,
}

impl Logjam {
    pub fn default(ctx: &egui::Context) -> Logjam {
        let tabs = PluginInstance::load_all_plugins();
        let plugins = egui_dock::DockState::new(tabs);

        let mut logjam = Logjam { plugins };
        if let Err(e) = logjam.load_preferences(ctx) {
            log::error!("{}", e);
        }

        logjam
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

impl Logjam {
    fn load_preferences(&mut self, ctx: &egui::Context) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("preferences.toml");
        let mut content = std::fs::read_to_string(file_path)?;

        content = content.replace("\\", "/");
        let doc = content.parse::<DocumentMut>()?;

        if let Some(theme) = doc
            .get("general")
            .and_then(|g| g.get("theme"))
            .and_then(|theme| theme.as_str())
        {
            match theme {
                "Light" => ctx.set_visuals(egui::Visuals::light()),
                "Dark" => ctx.set_visuals(egui::Visuals::dark()),
                "System" => match dark_light::detect()? {
                    dark_light::Mode::Light => ctx.set_visuals(egui::Visuals::light()),
                    _ => ctx.set_visuals(egui::Visuals::dark()),
                },
                _ => (),
            }
        }

        Ok(())
    }
}
