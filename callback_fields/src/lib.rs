mod callback_field;
mod manifest;

use std::path::PathBuf;

use abi_stable::std_types::RString;
use callback_field::CallbackField;
use logjam_core::ui::UiWrapper;
use eframe::egui::{self, Grid, Ui};
use manifest::Manifest;
use logjam_core::{LogjamPlugin, plugin::LogjamPlugin};

#[derive(LogjamPlugin)]
struct CallbackFieldsPlugin {
    _log_folder: PathBuf,
    callback_fields: Vec<CallbackField>,

    manifests: Vec<Manifest>,
    selected_manifest: usize
}

impl CallbackFieldsPlugin {
    fn default() -> Self {
        let log_folder = PathBuf::new();
        let manifests = manifest::Manifest::load_manifests().unwrap();

        let callback_fields = CallbackField::get_callback_fields();
        for callback_field in &callback_fields {
            callback_field.begin_search(&log_folder);
        }

        Self { _log_folder: log_folder, callback_fields, manifests, selected_manifest: 0 }
    }
}

impl LogjamPlugin for CallbackFieldsPlugin {
    fn title(&self) -> RString {
        RString::from("Callback Fields")
    }
    fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("manifests_combobox")
                .selected_text(self.manifests[self.selected_manifest].get_title())
                .show_ui(ui, |ui| {
                    for (i, manifest) in &mut self.manifests.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_manifest, i, manifest.get_title());
                    }
                })
        });

        Grid::new("callback_fields_grid")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                for callback_field in &self.callback_fields {
                    let response = ui.label(&callback_field.name);
                    if let Some(hint_text) = &callback_field.hint_text {
                        response.on_hover_text(hint_text);
                    }
                    ui.label(callback_field.result());
                    ui.end_row();
                }
            });
    }
}
