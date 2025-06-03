use crate::callback_field::CallbackField;
use crate::manifest::Manifest;
use std::ffi::CString;

use winapi::um::{winbase, winuser};

pub fn copy_plain(
    manifest: &Manifest,
    callback_fields: &Vec<CallbackField>,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        winuser::OpenClipboard(std::ptr::null_mut());
        winuser::EmptyClipboard();

        let plain_text = CString::new(manifest.to_plain_text(callback_fields))?;
        let len = plain_text.as_bytes_with_nul().len();
        let plain_hmem = {
            let hmem = winbase::GlobalAlloc(winbase::GMEM_MOVEABLE, len);
            let locked = winbase::GlobalLock(hmem);
            std::ptr::copy_nonoverlapping(plain_text.as_ptr(), locked as *mut i8, len);
            winbase::GlobalUnlock(hmem);
            hmem
        };

        winuser::SetClipboardData(1, plain_hmem);
    }

    Ok(())
}
