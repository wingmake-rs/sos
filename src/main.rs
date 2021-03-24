#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::led;

const LETTER_O: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
];
const LETTER_S: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
];
const LONG_LAPSE:  u32 = 654;
const SMALL_LAPSE: u32 = 250;

#[entry]
fn main() -> ! {
    if let Some(peripheral) = microbit::Peripherals::take() {
        let gpio = peripheral.GPIO.split();
        let mut delay = Delay::new(peripheral.TIMER0);
        let mut leds = led::Display::new(
            gpio.pin4.into_push_pull_output(),
            gpio.pin5.into_push_pull_output(),
            gpio.pin6.into_push_pull_output(),
            gpio.pin7.into_push_pull_output(),
            gpio.pin8.into_push_pull_output(),
            gpio.pin9.into_push_pull_output(),
            gpio.pin10.into_push_pull_output(),
            gpio.pin11.into_push_pull_output(),
            gpio.pin12.into_push_pull_output(),
            gpio.pin13.into_push_pull_output(),
            gpio.pin14.into_push_pull_output(),
            gpio.pin15.into_push_pull_output(),
        );

        loop {
            leds.display(&mut delay, LETTER_S, SMALL_LAPSE);
            leds.clear();
            delay.delay_ms(SMALL_LAPSE);
            leds.display(&mut delay, LETTER_S, SMALL_LAPSE);
            leds.clear();
            delay.delay_ms(SMALL_LAPSE);
            leds.display(&mut delay, LETTER_S, SMALL_LAPSE);
            leds.clear();

            delay.delay_ms(SMALL_LAPSE);

            leds.display(&mut delay, LETTER_O, LONG_LAPSE);
            leds.clear();
            delay.delay_ms(SMALL_LAPSE);
            leds.display(&mut delay, LETTER_O, LONG_LAPSE);
            leds.clear();
            delay.delay_ms(SMALL_LAPSE);
            leds.display(&mut delay, LETTER_O, LONG_LAPSE);
            leds.clear();

            delay.delay_ms(SMALL_LAPSE);

            leds.display(&mut delay, LETTER_S, SMALL_LAPSE);
            leds.clear();
            delay.delay_ms(SMALL_LAPSE);
            leds.display(&mut delay, LETTER_S, SMALL_LAPSE);
            leds.clear();
            delay.delay_ms(SMALL_LAPSE);
            leds.display(&mut delay, LETTER_S, SMALL_LAPSE);
            leds.clear();

            delay.delay_ms(LONG_LAPSE * 2);
        }
    }

    panic!("End");
}
