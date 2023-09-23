pub fn cpuid() -> usize {
    unsafe { crate::start::CPUID }
}