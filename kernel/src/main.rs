#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(naked_functions)]

use crate::proc::cpuid;

mod panic;
mod param;
mod proc;
mod start;
mod uart;

#[no_mangle]
extern "C" fn rust_main() -> ! {
    if cpuid() == 0 {
        uart::uart_init();
    } else {
        // Wait for init
    }
    loop {
        uart::write_uart_reg(0, 'c' as u8);
    }
}