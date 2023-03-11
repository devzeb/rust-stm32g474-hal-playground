use interfaces_hal::{ReadByte, WriteByte};
use stm32g4::stm32g474::LPUART1;
use crate::rcc::Rcc;

pub struct Lpuart1 {
    registers: LPUART1,
}

impl Lpuart1 {
    pub fn new(registers_: stm32g4::stm32g474::LPUART1, rcc: &mut Rcc) -> Lpuart1 {
        rcc.enable_clock_lpuart1();

        // registers_.cr1.write(|r| r.fifoen().set_bit());

        let uart = Lpuart1 { registers: registers_ };
        uart.set_baud();
        uart.enable();



        uart
    }

    fn enable(&self) {
        self.registers.cr1.modify(|_, w| w.ue().set_bit());
    }

    fn set_baud(&self) {
        // const SYSTEM_CLOCK: u32 = 32768;
        // const TARGET_BAUD: u32 = 9600;
        // const CLOCK_DIVIDER: u32 = SYSTEM_CLOCK / TARGET_BAUD;

        self.registers.brr.write(|w| unsafe { w.bits(426667) });
    }

    pub fn enable_receive(&self) {
        self.registers.cr1.modify(|_, w| w.re().set_bit());
    }

    pub fn enable_transmit(&self) {
        self.registers.cr1.modify(|_, w| w.te().set_bit());
    }
}

impl ReadByte for Lpuart1 {
    fn read_byte(&self) -> u8 {
        while self.registers.isr.read().rxne().bit_is_clear() {}

        let read_val = self.registers.rdr.read().bits();

        read_val as u8
    }
}

impl WriteByte for Lpuart1 {
    fn write_byte(&self, byte: u8)  {
        while self.registers.isr.read().txe().bit_is_clear() {}

        self.registers.tdr.write(|w| unsafe { w.bits(byte as u32) });

        // while self.registers.isr.read().tc().bit_is_clear() {}
    }
}

