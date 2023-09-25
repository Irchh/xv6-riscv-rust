#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![feature(thread_local)]

use core::sync::atomic::{AtomicBool, Ordering};
use crate::proc::cpuid;

mod panic;
mod param;
mod proc;
mod start;
mod uart;

static STARTED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
extern "C" fn rust_main() -> ! {
    if cpuid() == 0 {
        uart::uart_init();

        STARTED.store(true, Ordering::Relaxed)
    } else {
        while STARTED.load(Ordering::Relaxed) == false {}
    }
    let id = cpuid() as u8;
    loop {
        unsafe {
            uart::UART.lock().write_reg(0, b'0' + id)
        }
    }
}