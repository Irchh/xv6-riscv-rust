use core::ops::{BitAnd, Not};
use spin::Mutex;
use crate::kprintln;
use crate::memlayout::PHYSTOP;
use crate::riscv_defs::PGSIZE;

extern "C" {
    static end: u8; // First address after kernel, defined by linker.ld.
}

pub struct Run {
    next: *mut Run
}

pub struct KernelMem {
    freelist: *mut Run
}

impl KernelMem {
    pub const fn new() -> Self {
        Self {
            freelist: 0 as *mut Run,
        }
    }

    pub unsafe fn init(&mut self) {
        self.freerange(&end as *const u8 as *mut u8, PHYSTOP as *mut u8);
    }

    pub unsafe fn freerange(&mut self, pa_start: *mut u8, pa_end: *mut u8) {
        kprintln!("Freeing from {:?} to {:?}", pa_start, pa_end);
        let mut p: *mut u8 = Self::round_up(pa_start as usize) as *mut u8;
        while p.offset(PGSIZE as isize) <= pa_end {
            self.free(p);
            p = p.add(PGSIZE);
        }
    }

    /// Free the page of physical memory pointed at by pa,
    /// which normally should have been returned by a
    /// call to KernelMem::alloc().  (The exception is when
    /// initializing the allocator; see KernelMem::init above.)
    pub unsafe fn free(&mut self, page: *mut u8) {
        if (page as usize)%PGSIZE != 0 || page < &end as *const u8 as *mut u8 || page as usize >= PHYSTOP {
            panic!("free")
        }
        page.write_bytes(1, PGSIZE);

        let r = page as *mut Run;
        let mut run = r.read();
        run.next = self.freelist;
        r.write(run);
        self.freelist = r;
    }

    /// Allocate one 4096-byte page of physical memory.
    /// Returns a pointer that the kernel can use.
    /// Returns 0 if the memory cannot be allocated.
    pub fn alloc(&mut self) -> *mut u8 {
        let r = self.freelist;
        if !r.is_null() {
            // SAFETY: r is not null.
            unsafe {
                self.freelist = r.read().next;
                r.write_bytes(5, PGSIZE);
            }
        }

        r as *mut u8
    }

    fn round_up(a: usize) -> usize {
        (a + PGSIZE - 1).bitand((PGSIZE - 1).not())
    }
}
unsafe impl Send for KernelMem {}

pub static KMEM: Mutex<KernelMem> = Mutex::new(KernelMem::new());
