use stm32g4::stm32g474::RCC;

pub struct Rcc {
    registers_rcc: RCC,
}

impl Rcc {
    pub fn new(peripherals: RCC) -> Rcc {
        Rcc { registers_rcc: peripherals }
    }
}

impl Rcc {
    pub fn enable_clock_uart1(&mut self) {
        self.registers_rcc.apb2enr.write(|w| w.usart1en().set_bit());
    }

    pub fn enable_clock_lpuart1(&mut self) {
        self.registers_rcc.apb1enr2.write(|w| w.lpuart1en().set_bit());
    }
}