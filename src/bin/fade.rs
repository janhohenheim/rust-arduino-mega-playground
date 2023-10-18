#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);

    // Digital pin 5 is connected to a LED and a resistor in series
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer3);
    pwm_led.enable();


    // set up serial interface for text output
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        for x in (0..=255).chain((0..=254).rev()) {
            ufmt::uwriteln!(&mut serial, "brightness: {}\r", x).void_unwrap();
            pwm_led.set_duty(x);
            arduino_hal::delay_ms(10);
        }
    }
}