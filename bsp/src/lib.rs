#![no_std]

use stm32g4::stm32g474::Peripherals;

use stm32g474_hal::{rcc::Rcc, lpuart1::Lpuart1};

// let mut peripherals = stm32g474::;
//
// bsp::init(&mut peripherals);

pub struct Bsp {
    pub lpuart1: Lpuart1,
}

impl Bsp {
    pub fn new() -> Bsp {
        let per = Peripherals::take().unwrap();
        per.RCC.ahb2enr.write(|w|w.gpioaen().set_bit());

        let mut rcc = Rcc::new(per.RCC);


        // enable alternate function mode for
        per.GPIOA.moder.modify(|_,w| w.moder2().bits(0b10));
        per.GPIOA.moder.modify(|_,w| w.moder3().bits(0b10));

        // set alternate function 12 for GPIOA2 & GPIOA3
        per.GPIOA.afrl.modify(|_,w| w.afrl2().bits(12));
        per.GPIOA.afrl.modify(|_,w| w.afrl3().bits(12));

        let bsp = Bsp {
            // peripherals: per,
            lpuart1: Lpuart1::new( per.LPUART1, &mut rcc),
        };

        bsp.lpuart1.enable_receive();
        bsp.lpuart1.enable_transmit();

        bsp
    }
}
