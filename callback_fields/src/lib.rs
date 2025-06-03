mod callback_field;
mod copy_to_clipboard;
mod manifest;

use std::path::PathBuf;

use abi_stable::std_types::RString;
use callback_field::CallbackField;
use eframe::egui::{self, Grid, Ui};
use logjam_core::ui::UiWrapper;
use logjam_core::{LogjamPlugin, plugin::LogjamPlugin};
use manifest::Manifest;

#[derive(LogjamPlugin)]
struct CallbackFieldsPlugin {
    _log_folder: PathBuf,
    callback_fields: Vec<CallbackField>,

    manifests: Vec<Manifest>,
    selected_manifest: usize,

    selected_format: String,
}

impl CallbackFieldsPlugin {
    fn default() -> Self {
        let log_folder = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let manifests = manifest::Manifest::load_manifests().unwrap();

        let callback_fields = CallbackField::get_callback_fields();
        for callback_field in &callback_fields {
            callback_field.begin_search(&log_folder);
        }

        Self {
            _log_folder: log_folder,
            callback_fields,
            manifests,
            selected_manifest: 0,
            selected_format: "Plain".to_string(),
        }
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

        ui.horizontal(|ui| {
            ui.label("Click to copy description!");
            if ui.button("").clicked() {
                if let Err(e) = match self.selected_format.as_str() {
                    "Plain" => copy_to_clipboard::copy_plain(
                        &self.manifests[self.selected_manifest],
                        &self.callback_fields,
                    ),
                    "Jira" => Ok(()),
                    "ADO" => Ok(()),
                    "Hansoft" => Ok(()),
                    _ => Ok(()),
                } {
                    eprintln!("{}", e);
                }
            }
            ui.label("Copy for:");
            egui::ComboBox::from_id_salt("copy_formats")
                .selected_text(&self.selected_format)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_format, "Plain".to_string(), "Plain");
                });
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
