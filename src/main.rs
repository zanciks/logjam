#![allow(dead_code)]

mod plugin;
mod manifest;
mod callback_fields;

fn main() {
    let plugins = plugin::get_all_plugins();
    println!("{:?}", plugins);
}
