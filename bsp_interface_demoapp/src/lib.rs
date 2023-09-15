#![no_std]
use interfaces_hal::{ReadByte, WriteByte};
use crate::gpio_pin::{GpioOutputController};

pub mod gpio_pin;

pub trait BspInterfaceDemoapp {
    fn get_stdoutput(&self) -> &dyn WriteByte;
    fn get_stdinput(&self) -> &dyn ReadByte;
    fn get_gpio(&self) -> &dyn GpioOutputController;
}

