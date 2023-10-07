pub(crate) const PGSIZE: usize = 4096;
pub(crate) const PGSHIFT: usize = 12;

pub(crate) const PTE_V: usize = 1;
pub(crate) const PTE_R: usize = 1usize << 1;
pub(crate) const PTE_W: usize = 1usize << 2;
pub(crate) const PTE_X: usize = 1usize << 3;
pub(crate) const PTE_U: usize = 1usize << 4; // user can access

#[inline]
pub(crate) fn PA2PTE(pa: usize) -> usize {
    (pa >> 12) << 10
}
#[inline]
pub(crate) fn PTE2PA(pte: usize) -> usize {
    (pte >> 10) << 12
}

pub(crate) const PXMASK: usize = 0x1FF; // 9 bits
#[inline]
pub(crate) fn PXSHIFT(level: i32) -> usize {
    PGSHIFT + 9 * level as usize
}
#[inline]
pub(crate) fn PX(level: i32, virtual_address: usize) -> usize {
    (virtual_address>>PXSHIFT(level)) & PXMASK
}

/// one beyond the highest possible virtual address.
/// MAXVA is actually one bit less than the max allowed by
/// Sv39, to avoid having to sign-extend virtual addresses
/// that have the high bit set.
pub(crate) const MAXVA: usize = 1usize << (9 + 9 + 9 + 12 - 1);
