pub enum GpioPin {
    Led,
    LpuartRx,
    LpuartTx,
}

pub trait GpioOutputController {
    fn set_active(&self, pin: &GpioPin);
    fn set_inactive(&self, pin: &GpioPin);
}
