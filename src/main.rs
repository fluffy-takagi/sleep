#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::panic::PanicInfo;

use winapi::um::powrprof::SetSuspendState;
use winapi::um::processthreadsapi::ExitProcess;

#[no_mangle]
pub extern "C" fn mainCRTStartup() {
    unsafe {
        SetSuspendState(0, 1, 1);
        ExitProcess(0);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
