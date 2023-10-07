use spin::Mutex;
use crate::kalloc::KMEM;
use crate::memlayout::{KERNBASE, PHYSTOP, PLIC, TRAMPOLINE, UART0, VIRTIO0};
use crate::riscv_defs::{MAXVA, PA2PTE, PGSIZE, PTE2PA, PTE_R, PTE_V, PTE_W, PTE_X, PX};
use crate::util::round_down_4k;

extern "C" {
    static etext: u8; // First address after kernel, defined by linker.ld.
    static _trampoline: u8; // First address after kernel, defined by linker.ld.
}

type pagetable_t = *mut usize;
type pte_t = usize;

pub struct KVM {
    kernel_pagetable: pagetable_t,
}

impl KVM {
    pub const fn new() -> Self {
        Self {
            kernel_pagetable: 0 as pagetable_t,
        }
    }

    pub fn init(&mut self) {
        self.kernel_pagetable = self.make();
    }

    pub fn init_hart(&mut self) {
        todo!()
    }

    fn make(&mut self) -> pagetable_t {
        let kpgtbl = KMEM.lock().alloc() as *mut pte_t;
        if kpgtbl.is_null() {
            panic!("No memory");
        }
        // SAFETY: alloc allocates a whole page, and checked for null
        unsafe { kpgtbl.write_bytes(0, PGSIZE) }

        self.map(kpgtbl, UART0, UART0, PGSIZE, PTE_R | PTE_W);

        // virtio mmio disk interface
        self.map(kpgtbl, VIRTIO0, VIRTIO0, PGSIZE, PTE_R | PTE_W);

        // PLIC
        self.map(kpgtbl, PLIC, PLIC, 0x400000, PTE_R | PTE_W);

        let etext_usize = unsafe {&etext} as *const u8 as usize;
        // map kernel text executable and read-only.
        self.map(kpgtbl, KERNBASE, KERNBASE, etext_usize-KERNBASE, PTE_R | PTE_X);

        // map kernel data and the physical RAM we'll make use of.
        self.map(kpgtbl, etext_usize, etext_usize, PHYSTOP-etext_usize, PTE_R | PTE_W);

        // map the trampoline for trap entry/exit to
        // the highest virtual address in the kernel.
        self.map(kpgtbl, TRAMPOLINE, unsafe {&_trampoline } as *const u8 as usize, PGSIZE, PTE_R | PTE_X);

        //proc_mapstacks(kpgtbl)
        kpgtbl
    }

    fn map(&mut self, kpgtbl: *mut pte_t, virtual_address: usize, physical_address: usize, size: usize, perm: usize) {
        if self.mappages(kpgtbl, virtual_address, physical_address, size, perm).is_err() {
            panic!("KVM::map")
        }
    }

    fn mappages(&mut self, pagetable: *mut pte_t, virtual_address: usize, mut physical_address: usize, size: usize, perm: usize) -> Result<(), ()>{
        if size == 0 {
            panic!("KVM::mappages: size is 0.")
        }

        let mut a = round_down_4k(virtual_address);
        let last = round_down_4k(virtual_address + size - 1);

        loop {
            let pte = unsafe { self.walk(pagetable, a, true) };
            if pte.is_null() {
                return Err(());
            }
            // SAFETY: pointer is not null
            if unsafe { pte.read() } & PTE_V != 0 {
                panic!("mappages: remap")
            }
            unsafe {
                pte.write(((physical_address >> 12) << 10) | perm | PTE_V)
            }
            if a == last {
                break
            }
            a += PGSIZE;
            physical_address += PGSIZE;
        }

        Ok(())
    }
    unsafe fn walk(&self, mut pagetable: *mut pte_t, virtual_address: usize, alloc: bool) -> *mut pte_t {
        if virtual_address >= MAXVA {
            panic!("KVM::walk")
        }
        for level in (1..=2).rev() {
            let pte = pagetable.offset(PX(level, virtual_address) as isize);
            if pte.read() & PTE_V != 0 {
                pagetable = PTE2PA(pte.read()) as *mut usize;
            } else {
                if !alloc {
                    return 0 as *mut pte_t;
                }
                pagetable = KMEM.lock().alloc() as *mut pte_t;
                if pagetable.is_null() {
                    return pagetable;
                }
                pagetable.write_bytes(0, PGSIZE);
                pte.write(PA2PTE(pagetable as usize) | PTE_V)
            }
        }
        pagetable.offset(PX(0, virtual_address) as isize)
    }
}
unsafe impl Send for KVM {}

pub static KVM: Mutex<KVM> = Mutex::new(KVM::new());
