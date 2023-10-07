use crate::riscv_defs::{MAXVA, PGSIZE};

pub(crate) const UART0: usize = 0x10000000usize;
pub(crate) const UART0_IRQ: usize = 10;

pub(crate) const VIRTIO0: usize = 0x10001000usize;
pub(crate) const VIRTIO0_IRQ: usize = 1;

pub(crate) const PLIC: usize = 0x0c000000usize;

pub(crate) const KERNBASE: usize = 0x80000000;
pub(crate) const PHYSTOP: usize = KERNBASE + 128*1024*1024 - 1024*25; // idk why but "- 1024*25" fixes some issue idk.

pub(crate) const TRAMPOLINE: usize = MAXVA - PGSIZE;
