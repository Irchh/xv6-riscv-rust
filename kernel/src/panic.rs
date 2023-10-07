use core::panic::PanicInfo;
use crate::kprintln;
use crate::uart::UART;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe {
        UART.force_unlock()
    };
    kprintln!("\n{}", panic_info);
    loop {}
}