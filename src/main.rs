#![allow(dead_code)]

mod callback_fields;
mod manifest;
mod plugin;

fn main() {
    let plugins = plugin::Plugin::get_all_plugins();
    println!("{:?}", plugins);
}
