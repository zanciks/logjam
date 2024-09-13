#![allow(dead_code)]

mod plugin;
mod manifest;
mod callback_fields;

fn main() {
    let _ = plugin::get_all_plugins();
}
