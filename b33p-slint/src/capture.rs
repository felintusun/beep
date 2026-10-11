//! As Signal Desktop's "Screen security" (2025-05): the window is excluded from screen capture
//! and Windows Recall.

/// Call once the window is shown: it needs the native handle.
#[cfg(windows)]
pub fn exclude(window: &slint::Window) {
    if let Err(error) = set_display_affinity(window) {
        tracing::warn!(%error, "window not excluded from screen capture");
    }
}

#[cfg(windows)]
fn set_display_affinity(window: &slint::Window) -> Result<(), Box<dyn std::error::Error>> {
    use raw_window_handle::{HasWindowHandle as _, RawWindowHandle};
    use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE},
    };
    let RawWindowHandle::Win32(win32) = window.window_handle().window_handle()?.as_raw() else {
        return Err("not a Win32 window".into());
    };
    let hwnd = HWND(win32.hwnd.get() as *mut core::ffi::c_void);
    unsafe { SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE) }?;
    Ok(())
}

#[cfg(not(windows))]
pub fn exclude(_: &slint::Window) {}
