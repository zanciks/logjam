#![allow(dead_code)]

mod callback_fields;
mod manifest;
mod plugin;

fn main() {
    let plugins = plugin::Plugin::get_all_plugins();
    for plugin in &plugins {
        plugin.begin_search();
    }
}
