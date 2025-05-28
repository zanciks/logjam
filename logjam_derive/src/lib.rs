use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LogjamPlugin)]
pub fn add_test_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn init() -> *mut std::ffi::c_void {
            let plugin: Box<dyn LogjamPlugin> = Box::new(#struct_name::default());
            Box::into_raw(plugin) as *mut std::ffi::c_void
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn render(ptr: *mut std::ffi::c_void, ui: &logjam_core::ui::UiWrapper) {
            if !ptr.is_null() {
                let plugin = unsafe { &mut *(ptr as *mut #struct_name) as &mut dyn LogjamPlugin };
                plugin.render(ui);
            }
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn title(ptr: *mut std::ffi::c_void) -> RString {
            if !ptr.is_null() {
                let plugin = unsafe { &mut *(ptr as *mut #struct_name) as &mut dyn LogjamPlugin };
                plugin.title()
            } else { panic!() }
        }
    };

    TokenStream::from(expanded)
}