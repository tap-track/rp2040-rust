use core::fmt::Write;
use embedded_hal::delay::DelayNs;

pub trait Display {
    fn init(&mut self);
    fn show_text(&mut self, text: &str);
}

pub struct Lcd1602<I2C, DELAY> {
    i2c_bus: I2C,
    delay: DELAY,
}

impl<I2C, DELAY> Lcd1602<I2C, DELAY>
where
    I2C: Write,
    DELAY: DelayNs,
{
    const I2C_ADDR: u8 = 0x27;
}

impl<I2C, DELAY> Display for Lcd1602<I2C, DELAY> {
    fn init(&mut self) {}

    fn show_text(&mut self, text: &str) {}
}
