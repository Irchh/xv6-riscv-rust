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
mod kprintln;
mod kalloc;
mod memlayout;
mod vm;
mod util;
mod riscv_defs;

static STARTED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
extern "C" fn rust_main() -> ! {
    if cpuid() == 0 {
        uart::uart_init();
        kprintln!("xv6 kernel is booting");
        unsafe { kalloc::KMEM.lock().init() }   // Physical page allocator
        vm::KVM.lock().init();                  // Create kernel page table
        //vm::KVM.lock().init_hart();             // Turn on paging
        STARTED.store(true, Ordering::Relaxed)
    } else {
        while STARTED.load(Ordering::Relaxed) == false {}
        kprintln!("Hello from hart {}", cpuid());
    }
    loop {}
}