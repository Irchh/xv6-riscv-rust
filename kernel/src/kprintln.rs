use core::fmt;
use core::fmt::Write;
use crate::uart;
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::kprintln::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprintln::_print("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    // TODO: Make sure interrupts are disabled
    uart::UART.lock().write_fmt(args).unwrap();
}