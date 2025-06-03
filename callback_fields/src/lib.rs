mod callback_field;

use std::path::PathBuf;

use abi_stable::std_types::RString;
use callback_field::CallbackField;
use eframe::egui::{Grid, Ui};
use logjam_core::{LogjamPlugin, plugin::LogjamPlugin};

#[derive(LogjamPlugin)]
struct CallbackFieldsPlugin {
    _log_folder: PathBuf,
    callback_fields: Vec<CallbackField>
}

impl CallbackFieldsPlugin {
    fn default() -> Self {
        let log_folder = PathBuf::new();

        let callback_fields = CallbackField::get_callback_fields();
        for callback_field in &callback_fields {
            callback_field.begin_search(&log_folder);
        }

        Self { _log_folder: log_folder, callback_fields }
    }
}

impl LogjamPlugin for CallbackFieldsPlugin {
    fn title(&self) -> RString {
        RString::from("Callback Fields")
    }
    fn render(&mut self, ui: &mut Ui) {
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
