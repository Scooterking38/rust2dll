use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;

#[no_mangle]
pub extern "C" fn hello() {
    unsafe {
        MessageBoxA(0, b"Hello\0".as_ptr(), b"Rust DLL\0".as_ptr(), 0);
    }
}
