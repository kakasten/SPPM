#![no_std]
#![no_main]
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;

use panic_halt as _;
use rp2040_hal as hal;
use hal::pac;
use embedded_hal::digital::v2::{OutputPin, InputPin};
use rp2040_hal::clocks::Clock;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let button_one_pin = pins.gpio16.into_pull_up_input();
    let button_two_pin = pins.gpio17.into_pull_up_input();
    let sensor_pin = pins.gpio18.into_pull_up_input();

    let mut close_door_pin = pins.gpio21.into_push_pull_output();
    let mut close_valve_pin = pins.gpio22.into_push_pull_output();

    loop {
        if button_one_pin.is_low().unwrap() && button_two_pin.is_low().unwrap(){
            close_door_pin.set_high().unwrap();
            delay.delay_ms(1500);
            loop {
                if sensor_pin.is_high().unwrap(){
                    close_valve_pin.set_high().unwrap();
                    delay.delay_ms(2000);
                    close_valve_pin.set_low().unwrap();
                    delay.delay_ms(500);
                    close_door_pin.set_low().unwrap();
                    break;
                }
            }
        }
    }
}
