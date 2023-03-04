#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

mod morse;
use morse::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 19200);

    
    ufmt::uwriteln!(&mut serial, "Hello from Arduino Morse Echo written in Rust btw!\r").void_unwrap();
    
    let mut led = pins.d13.into_output();
    let mut mw = MorseWriter::new(led);


    let mut one_line = false;
    loop {
        if !one_line {
            ufmt::uwrite!(&mut serial, "?: ").void_unwrap();
            one_line = true;
        }

        let c = nb::block!(serial.read()).void_unwrap() as char;
        match c {
            'a'..='z' => {
                // led.set_high();
                ufmt::uwrite!(&mut serial, "{} ", 
                morse::ALPHABET[(c as u8 - b'a') as usize].1).void_unwrap();
                mw.write(c);
                // led.set_low();
            },
            '\n' => {
                ufmt::uwriteln!(&mut serial, "\r").void_unwrap();
                serial.flush();
                one_line = false;
            },
            _ => {
                // led.set_high();
                ufmt::uwriteln!(&mut serial, "Invalid character\r").void_unwrap();
                // led.set_low();
            }
        }
    }
}
