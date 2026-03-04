#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::{
    self,
    pac::{self},
    Clock,
};

use crate::input::Button;
mod display;
mod input;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn main() -> ! {
    //Initial pac / peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = rp2040_hal::Watchdog::new(pac.WATCHDOG);

    let clock = rp2040_hal::clocks::init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clock.system_clock.freq().to_Hz());

    let sio = rp2040_hal::Sio::new(pac.SIO);

    //Initial Pins / GPIO
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut i_led = pins.gpio25.into_push_pull_output();
    let display_sda_gpio = pins.gpio0.reconfigure();
    let display_scl_gpio = pins.gpio1.reconfigure();

    let state_button_gpio = pins.gpio6.into_pull_up_input();
    let category_button_gpio = pins.gpio7.into_pull_up_input();

    let mut state_button = Button::new(state_button_gpio);
    let mut category_button = Button::new(category_button_gpio);

    let mut i2c = rp2040_hal::I2C::i2c0(
        pac.I2C0,
        display_sda_gpio,
        display_scl_gpio,
        400.kHz(),
        &mut pac.RESETS,
        &clock.system_clock,
    );

    loop {
        i_led.set_high().unwrap();
        delay.delay_ms(1 * 1000);
        i_led.set_low().unwrap();
        delay.delay_ms(1 * 1000);

        i2c.write_iter(0x2Cu8, [1]).unwrap();
    }
}
