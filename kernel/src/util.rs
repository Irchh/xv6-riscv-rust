use crate::riscv_defs::PGSIZE;

#[inline]
pub fn round_down_4k(a: usize) -> usize {
    a & !(PGSIZE-1)
}