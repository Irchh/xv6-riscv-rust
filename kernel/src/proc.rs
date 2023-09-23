use core::arch::asm;

pub fn cpuid() -> usize {
    let id;
    unsafe {
        asm!(
        "mv tp, {0}",
        out(reg) id
        );
    }
    id
}