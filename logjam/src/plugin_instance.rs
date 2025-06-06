use abi_stable::std_types::RString;
use libloading::{Library, Symbol};
use logjam_core::ui::UiWrapper;
use std::ffi::c_void;
use std::sync::Arc;

pub struct PluginInstance {
    pub title: String,
    ptr: *mut c_void,
    render: Symbol<'static, unsafe fn(*mut c_void, &UiWrapper)>,
    _lib: Arc<Library>,
}

impl PluginInstance {
    pub fn load(path: &str) -> PluginInstance {
        unsafe {
            let library = Arc::new(Library::new(path).unwrap());
            let lib_ref: &'static Library = &*Box::leak(Box::new(Arc::clone(&library)));

            let init: Symbol<unsafe fn() -> *mut c_void> = lib_ref.get(b"init").unwrap();
            let render: Symbol<unsafe fn(*mut c_void, &UiWrapper)> =
                lib_ref.get(b"render").unwrap();
            let title_function: Symbol<unsafe fn(*mut c_void) -> RString> =
                lib_ref.get(b"title").unwrap();

            let ptr = init();
            let title = title_function(ptr).to_string();

            log::info!("Loading plugin with name {}", &title);

            PluginInstance {
                title,
                ptr,
                render,
                _lib: library,
            }
        }
    }
    pub fn load_all_plugins() -> Vec<PluginInstance> {
        let mut plugins = vec![];
        let exe_path = std::env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        let plugins_path = exe_dir.join("plugins");

        for entry in std::fs::read_dir(plugins_path).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "dll") {
                plugins.push(PluginInstance::load(path.to_str().unwrap()))
            }
        }

        plugins
    }
    pub fn render(&self, ui: &UiWrapper) {
        unsafe { (self.render)(self.ptr, ui) }
    }
}
