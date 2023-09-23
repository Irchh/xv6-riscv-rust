use core::arch::{asm, global_asm};
use crate::param::*;

global_asm!(include_str!("asm/entry.S"));

#[no_mangle]
static mut STACK0: [u8; 4096*NCPU] = [0; 4096*NCPU];
// Needed for easy storage of thread specific data, like the hart id.
#[no_mangle]
static mut THREAD_LOCAL_STORAGE: [u8; 4096*NCPU] = [0; 4096*NCPU];

#[thread_local]
pub static mut CPUID: usize = 0;

#[no_mangle]
extern "C" fn rust_start() -> ! {
    // set M Previous Privilege mode to Supervisor, for mret.
    unsafe { riscv::register::mstatus::set_mpp(riscv::register::mstatus::MPP::Supervisor); }

    // set M Exception Program Counter to main, for mret.
    // requires gcc -mcmodel=medany (is this true for rust?)
    riscv::register::mepc::write(crate::rust_main as usize);

    // disable paging for now
    riscv::register::satp::write(0);

    // delegate all interrupts and exceptions to supervisor mode.
    // w_medeleg(0xffff);
    // w_mideleg(0xffff);
    unsafe {
        riscv::register::sie::set_sext();
        riscv::register::sie::set_stimer();
        riscv::register::sie::set_ssoft();
    }

    // configure Physical Memory Protection to give supervisor mode
    // access to all of physical memory.
    riscv::register::pmpaddr0::write(0x3fffffffffffffusize);
    riscv::register::pmpcfg0::write(0xf);

    // ask for clock interrupts.
    //timerinit();

    // keep each CPU's hartid in its tp register, for cpuid().
    let id = riscv::register::mhartid::read();
    unsafe { CPUID = id; }

    // switch to supervisor mode and jump to main().
    unsafe {
        asm!("mret");
    }

    loop {}
}