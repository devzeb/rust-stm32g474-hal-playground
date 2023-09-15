use bsp_interface_demoapp::gpio_pin::{GpioPin, GpioOutputController};
use stm32g474_hal::gpioa::{AlternateFunctionsGpioAPin2, AlternateFunctionsGpioAPin3, DriverGpioA, GpioAlternateFunctions, GpioPinMode, PinGpioA};

enum ModeOrAlternateFunction {
    Mode(GpioPinMode),
    AlternateFunction(GpioAlternateFunctions),
}

enum GpioPinInfo<'a> {
    PinGpioA {
        driver: &'a DriverGpioA,
        pin_number: PinGpioA,
        mode_or_alternate_function: ModeOrAlternateFunction,
    }
}

pub struct GpioDriverManager {
    pub(crate) driver_gpioa: DriverGpioA,
}

impl GpioDriverManager {
    fn get_pin_info(&self, pin: &GpioPin) -> GpioPinInfo {
        match pin {
            GpioPin::Led => {
                GpioPinInfo::PinGpioA {
                    driver: &self.driver_gpioa,
                    pin_number: PinGpioA::Pin5,
                    mode_or_alternate_function: ModeOrAlternateFunction::Mode(GpioPinMode::Output),
                }
            }
            GpioPin::LpuartRx => {
                GpioPinInfo::PinGpioA {
                    driver: &self.driver_gpioa,
                    pin_number: PinGpioA::Pin2,
                    mode_or_alternate_function: ModeOrAlternateFunction::AlternateFunction(GpioAlternateFunctions::Pin2 { af: AlternateFunctionsGpioAPin2::LpUartRx }),
                }
            }
            GpioPin::LpuartTx => {
                GpioPinInfo::PinGpioA {
                    driver: &self.driver_gpioa,
                    pin_number: PinGpioA::Pin3,
                    mode_or_alternate_function: ModeOrAlternateFunction::AlternateFunction(GpioAlternateFunctions::Pin3 { af: AlternateFunctionsGpioAPin3::LpUartTx }),
                }
            }
        }
    }

    pub fn initialize_gpio_pins(&self) {
        for gpio in [
            GpioPin::Led,
            GpioPin::LpuartRx,
            GpioPin::LpuartTx,
        ].iter() {
            match self.get_pin_info(gpio) {
                GpioPinInfo::PinGpioA { driver, pin_number, mode_or_alternate_function } => {
                    match mode_or_alternate_function {
                        ModeOrAlternateFunction::Mode(mode) => {
                            driver.set_mode(&pin_number, mode)
                        }
                        ModeOrAlternateFunction::AlternateFunction(alternate_function) => {
                            driver.enable_pin_alternate_function(alternate_function)
                        }
                    }
                }
            }
        }
    }
}

impl GpioOutputController for GpioDriverManager {
    fn set_active(&self, pin: &GpioPin) {
        match self.get_pin_info(pin) {
            GpioPinInfo::PinGpioA {
                driver,
                pin_number,
                ..
            } => {
                driver.set_pin(&pin_number, true)
            }
        }
    }

    fn set_inactive(&self, pin: &GpioPin) {
        match self.get_pin_info(pin) {
            GpioPinInfo::PinGpioA {
                driver,
                pin_number,
                ..
            } => {
                driver.reset_pin(&pin_number, true)
            }
        }
    }
}
