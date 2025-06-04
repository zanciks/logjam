use crate::callback_field::CallbackField;
use crate::manifest::Manifest;
use std::ffi::{CString, OsStr};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

use winapi::um::{winbase, winnt, winuser};

pub fn copy_plain(
    manifest: &Manifest,
    callback_fields: &Vec<CallbackField>,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        winuser::OpenClipboard(std::ptr::null_mut());
        winuser::EmptyClipboard();

        let plain_text = manifest.to_plain_text(callback_fields);
        let plain_hmem = generate_hmem(&plain_text)?;

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

        let plain_text = manifest.to_plain_text(callback_fields);
        let plain_hmem = generate_hmem(&plain_text)?;

        let hansoft_text = manifest.to_hansoft(callback_fields);
        let hansoft_hmem = generate_hmem(&hansoft_text)?;

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

pub fn copy_jira(
    manifest: &Manifest,
    callback_fields: &Vec<CallbackField>,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        winuser::OpenClipboard(std::ptr::null_mut());
        winuser::EmptyClipboard();

        // let plain_text = manifest.to_plain_text(callback_fields);
        // let plain_hmem = generate_hmem(&plain_text)?;

        let jira_text = manifest.to_jira(callback_fields);
        let jira_hmem = generate_hmem(&jira_text)?;

        winuser::SetClipboardData(1, jira_hmem);
        // winuser::SetClipboardData(1, plain_hmem);

        winuser::CloseClipboard();
    }

    Ok(())
}

unsafe fn generate_hmem(text: &str) -> Result<winnt::HANDLE, Box<dyn std::error::Error>> {
    unsafe {
        let text = CString::new(text)?;
        let len = text.as_bytes_with_nul().len();
        let hmem = {
            let hmem = winbase::GlobalAlloc(winbase::GMEM_MOVEABLE, len);
            let locked = winbase::GlobalLock(hmem);
            std::ptr::copy_nonoverlapping(text.as_ptr(), locked as *mut i8, len);
            winbase::GlobalUnlock(hmem);
            hmem
        };

        Ok(hmem)
    }
}
