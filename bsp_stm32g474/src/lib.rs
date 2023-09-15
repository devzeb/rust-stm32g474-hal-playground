#![no_std]

use bsp_interface_demoapp::{BspInterfaceDemoapp};

use stm32g4::stm32g474::{Peripherals};
use bsp_interface_demoapp::gpio_pin::{GpioOutputController};

use interfaces_hal::{ReadByte, WriteByte};

use stm32g474_hal::{lpuart1::Lpuart1, rcc::Rcc, gpioa::DriverGpioA};
use crate::gpio_driver_manager::{GpioDriverManager};


mod gpio_driver_manager;

pub struct Bsp {
    lpuart1: Lpuart1,
    gpio_driver_manager: GpioDriverManager,
}

impl Bsp {
    pub fn new() -> Bsp {
        let per = Peripherals::take().unwrap();
        let mut rcc = Rcc::new(per.RCC);

        let bsp = Bsp {
            // peripherals: per,
            lpuart1: Lpuart1::new(per.LPUART1, &mut rcc),
            gpio_driver_manager: GpioDriverManager { driver_gpioa: DriverGpioA::new(per.GPIOA, &mut rcc) },
        };

        bsp.gpio_driver_manager.initialize_gpio_pins();

        bsp.lpuart1.enable_receive();
        bsp.lpuart1.enable_transmit();

        bsp
    }
}


impl BspInterfaceDemoapp for Bsp {
    fn get_stdoutput(&self) -> &dyn WriteByte {
        &self.lpuart1
    }

    fn get_stdinput(&self) -> &dyn ReadByte {
        &self.lpuart1
    }

    fn get_gpio(&self) -> &dyn GpioOutputController {
        &self.gpio_driver_manager
    }
}
