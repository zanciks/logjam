use crate::callback_field::CallbackField;
use crate::manifest::Manifest;
use std::ffi::{CString, OsStr};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

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

        winuser::CloseClipboard();
    }

    Ok(())
}

pub fn copy_hansoft(
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

        let hansoft_text = CString::new(manifest.to_hansoft(callback_fields))?;
        let len = hansoft_text.as_bytes_with_nul().len();
        let hansoft_hmem = {
            let hmem = winbase::GlobalAlloc(winbase::GMEM_MOVEABLE, len);
            let locked = winbase::GlobalLock(hmem);
            std::ptr::copy_nonoverlapping(hansoft_text.as_ptr(), locked as *mut i8, len);
            winbase::GlobalUnlock(hmem);
            hmem
        };

        let wide: Vec<u16> = OsStr::new("application/vnd.hansoft.taggedtext")
            .encode_wide()
            .chain(once(0))
            .collect();
        let hansoft_format = winuser::RegisterClipboardFormatW(wide.as_ptr());

        winuser::SetClipboardData(hansoft_format, hansoft_hmem);
        winuser::SetClipboardData(1, plain_hmem);

        winuser::CloseClipboard();
    }

    Ok(())
}
