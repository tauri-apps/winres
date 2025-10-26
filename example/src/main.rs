use std::ffi::CString;

use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONINFORMATION, MB_OK};

fn main() {
    let lp_text = CString::new("Hello, world!").unwrap();
    let lp_caption = CString::new("MessageBox Example").unwrap();
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            lp_text.as_ptr() as *const u8,
            lp_caption.as_ptr() as *const u8,
            MB_OK | MB_ICONINFORMATION,
        );
    }
}
