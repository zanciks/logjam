mod callback_field;
mod copy_to_clipboard;
mod manifest;

use std::path::PathBuf;

use abi_stable::std_types::RString;
use callback_field::CallbackField;
use eframe::egui::{self, Align, Layout, Ui};
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::{FileDialog, FileDialogConfig};
use logjam_core::ui::UiWrapper;
use logjam_core::{LogjamPlugin, plugin::LogjamPlugin};
use manifest::Manifest;
use toml_edit::{DocumentMut, value};

#[derive(LogjamPlugin)]
struct CallbackFieldsPlugin {
    log_folder: Option<PathBuf>,
    callback_fields: Vec<CallbackField>,

    manifests: Vec<Manifest>,
    selected_manifest: usize,

    selected_format: String,
    file_dialog: FileDialog,
}

impl Default for CallbackFieldsPlugin {
    fn default() -> Self {
        let log_folder = None;
        let manifests = Manifest::load_manifests().unwrap();
        let callback_fields = CallbackField::get_callback_fields();

        let file_dialog = FileDialog::with_config(FileDialogConfig {
            as_modal: true,
            title_bar: false,
            default_size: egui::Vec2::new(700.0, 350.0),
            anchor: Some((egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)),
            ..Default::default()
        });

        let mut plugin = Self {
            log_folder,
            callback_fields,
            manifests,
            selected_manifest: 0,
            selected_format: "Plain Text".to_string(),
            file_dialog,
        };

        if let Err(e) = plugin.load_preferences() {
            log::error!("{}", e)
        }

        for callback_field in &plugin.callback_fields {
            // needs to be called after loading preferences
            callback_field.begin_search(&plugin.log_folder);
        }

        plugin
    }
}

impl LogjamPlugin for CallbackFieldsPlugin {
    fn title(&self) -> RString {
        RString::from("Callback Fields")
    }
    fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Selected template:");
            egui::ComboBox::from_id_salt("manifests_combobox")
                .selected_text(self.manifests[self.selected_manifest].get_title())
                .show_ui(ui, |ui| {
                    for (i, manifest) in &mut self.manifests.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_manifest, i, manifest.get_title());
                    }
                });
            ui.label("Click to select log folder:");
            if ui.button("Open").clicked() {
                self.file_dialog.pick_directory();
            }
        });

        self.file_dialog.update(ui.ctx());
        if let Some(path) = self.file_dialog.take_picked() {
            self.log_folder = Some(path.to_path_buf());
            for callback_field in &self.callback_fields {
                callback_field.stop_search();
            }
            self.callback_fields = CallbackField::get_callback_fields();
            for callback_field in &self.callback_fields {
                callback_field.begin_search(&self.log_folder);
            }
            if let Err(e) = self.save_preferences() {
                log::error!("{}", e)
            }
        }

        ui.horizontal(|ui| {
            ui.label("Click to copy description!");
            if ui.button("📋").clicked() {
                if let Err(e) = match self.selected_format.as_str() {
                    "Plain Text" => copy_to_clipboard::copy_plain(
                        &self.manifests[self.selected_manifest],
                        &self.callback_fields,
                    ),
                    "Jira" => copy_to_clipboard::copy_jira(
                        &self.manifests[self.selected_manifest],
                        &self.callback_fields,
                    ),
                    // "Jira (old)" => Ok(()),
                    // "ADO" => Ok(()),
                    "Hansoft" => copy_to_clipboard::copy_hansoft(
                        &self.manifests[self.selected_manifest],
                        &self.callback_fields,
                    ),
                    _ => Ok(()),
                } {
                    log::error!("{}", e);
                }
            }

            ui.label("Copy Format:");
            egui::ComboBox::from_id_salt("copy_formats")
                .selected_text(&self.selected_format)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected_format,
                        "Plain Text".to_string(),
                        "Plain Text",
                    );
                    ui.selectable_value(&mut self.selected_format, "Jira".to_string(), "Jira");
                    // ui.selectable_value(&mut self.selected_format, "Jira (old)".to_string(), "Jira (old)");
                    // ui.selectable_value(&mut self.selected_format, "ADO".to_string(), "ADO");
                    ui.selectable_value(
                        &mut self.selected_format,
                        "Hansoft".to_string(),
                        "Hansoft",
                    );
                });
        });

        ui.separator();

        TableBuilder::new(ui)
            .striped(true)
            .column(Column::initial(150.0))
            .column(Column::remainder())
            .body(|body| {
                body.rows(15.0, self.callback_fields.len(), |mut row| {
                    let callback_field = &self.callback_fields[row.index()];
                    row.col(|ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            let response = ui.label(&callback_field.name);
                            if let Some(hint_text) = &callback_field.hint_text {
                                response.on_hover_text(hint_text);
                            }
                        });
                    });
                    row.col(|ui| {
                        ui.label(callback_field.result());
                    });
                });
            });
    }
}

impl CallbackFieldsPlugin {
    fn save_preferences(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("preferences.toml");
        let mut content = std::fs::read_to_string(&file_path)?;

        content = content.replace("\\", "/");
        let mut doc = content.parse::<DocumentMut>()?;

        if let Some(log_folder) = &self.log_folder {
            doc["callback_fields"]["file_location"] = value(log_folder.to_string_lossy().as_ref());
        }

        std::fs::write(&file_path, doc.to_string())?;
        Ok(())
    }
    fn load_preferences(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("preferences.toml");
        let mut content = std::fs::read_to_string(&file_path)?;

        content = content.replace("\\", "/");
        let doc = content.parse::<DocumentMut>()?;

        if let Some(log_folder) = doc
            .get("callback_fields")
            .and_then(|cf| cf.get("file_location"))
            .and_then(|fl| fl.as_str())
        {
            self.log_folder = Some(log_folder.into());
        }

        Ok(())
    }
}
