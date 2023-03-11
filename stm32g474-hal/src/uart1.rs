use interfaces_hal::{ReadByte, WriteByte};
use stm32g4::stm32g474::USART1;
use crate::rcc::Rcc;

pub struct Uart1 {
    registers: USART1,
}

impl Uart1 {
    pub fn new(usart1: stm32g4::stm32g474::USART1, rcc: &mut Rcc) -> Uart1 {
        rcc.enable_clock_uart1();

        let uart = Uart1 { registers: usart1 };
        uart.set_baud();
        uart.enable();

        uart
    }

    fn enable(&self) {
        self.registers.cr1.modify(|_, w| w.ue().set_bit());
    }

    fn set_baud(&self) {
        const SYSTEM_CLOCK: u32 = 16000000;
        const TARGET_BAUD: u32 = 9600;
        const CLOCK_DIVIDER: u32 = SYSTEM_CLOCK / TARGET_BAUD;

        self.registers.brr.write(|w| unsafe { w.bits(CLOCK_DIVIDER) });
    }

    pub fn enable_receive(&self) {
        self.registers.cr1.modify(|_, w| w.re().set_bit());
    }

    pub fn enable_transmit(&self) {
        self.registers.cr1.modify(|_, w| w.te().set_bit());
    }
}

impl ReadByte for Uart1 {
    fn read_byte(&self) -> u8 {
        while self.registers.isr.read().rxne().bit_is_clear() {}

        let read_val = self.registers.rdr.read().bits();

        read_val as u8
    }
}

impl WriteByte for Uart1 {
    fn write_byte(&self, byte: u8)  {
        while self.registers.isr.read().txe().bit_is_clear() {}

        self.registers.tdr.write(|w| unsafe { w.bits(byte as u32) });

    }
}

