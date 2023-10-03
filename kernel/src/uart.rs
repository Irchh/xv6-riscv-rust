#![allow(dead_code)]

use core::fmt;
use spin::Mutex;

const UART0: usize = 0x10000000;

const RHR: usize = 0;            // receive holding register (for input bytes)
const THR: usize = 0;            // transmit holding register (for output bytes)
const IER: usize = 1;            // interrupt enable register
const IER_RX_ENABLE: u8 = 1<<0;
const IER_TX_ENABLE: u8 = 1<<1;
const FCR: usize = 2;            // FIFO control register
const FCR_FIFO_ENABLE: u8 = 1<<0;
const FCR_FIFO_CLEAR: u8 = 3<<1; // clear the content of the two FIFOs
const ISR: usize = 2;            // interrupt status register
const LCR: usize = 3;            // line control register
const LCR_EIGHT_BITS: u8 = 3<<0;
const LCR_BAUD_LATCH: u8 = 1<<7; // special mode to set baud rate
const LSR: usize = 5;            // line status register
const LSR_RX_READY: u8   = 1<<0;   // input is waiting to be read from RHR
const LSR_TX_IDLE: u8 = 1<<5;    // THR can accept another character to send

pub static UART: Mutex<Uart> = Mutex::new(Uart::new());

pub struct Uart {}

impl Uart {
    pub const fn new() -> Self {
        Uart {}
    }

    pub unsafe fn init(&self) {
        // disable interrupts.
        self.write_reg(IER, 0x00);

        // special mode to set baud rate.
        self.write_reg(LCR, LCR_BAUD_LATCH);

        // LSB for baud rate of 38.4K.
        self.write_reg(0, 0x03);

        // MSB for baud rate of 38.4K.
        self.write_reg(1, 0x00);

        // leave set-baud mode,
        // and set word length to 8 bits, no parity.
        self.write_reg(LCR, LCR_EIGHT_BITS);

        // reset and enable FIFOs.
        self.write_reg(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);

        // enable transmit and receive interrupts.
        self.write_reg(IER, IER_TX_ENABLE | IER_RX_ENABLE);
    }

    pub fn putc(&self, ch: u8) {
        // wait for Transmit Holding Empty to be set in LSR.
        while unsafe { self.read_reg(LSR) } & LSR_TX_IDLE == 0 {}
        unsafe {
            self.write_reg(0, ch)
        }
    }

    pub fn puts(&self, s: &str) {
        for ch in s.chars() {
            if ch.is_ascii() {
                self.putc(ch as u8)
            } else {
                self.putc(b'?')
            }
        }
    }

    unsafe fn write_reg(&self, reg: usize, value: u8) {
        let addr = (UART0 + reg) as *mut u8;
        addr.write(value);
    }

    unsafe fn read_reg(&self, reg: usize) -> u8 {
        let addr = (UART0 + reg) as *mut u8;
        addr.read()
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}

pub fn uart_init() {
    unsafe {
        let uartlock = UART.lock();
        uartlock.init();
        uartlock.puts("Uart init\n");
    }
}