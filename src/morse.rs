// struct MyDevice<EN, RS, RW> {
//     en: EN,
//     rs: RS,
//     rw: RW
// }

// impl<EN: OutputPin, RS: OutputPin, RW: OutputPin>
// MyDevice<EN, RS, RW> {
//     pub fn new(en: EN, rs: RS, rw: RW) -> Self {
//         MyDevice { en, rs, rw }
//     }

//     pub fn enable(&mut self) {
//         self.rs.set_low();
//         self.rw.set_low();
//         self.en.set_high();
//     }
// }

pub const ALPHABET: [(char, &str); 26] = [
    ('A', ".-"),
    ('B', "-..."),
    ('C', "-.-."),
    ('D', "-.."),
    ('E', "."),
    ('F', "..-."),
    ('G', "--."),
    ('H', "...."),
    ('I', ".."),
    ('J', ".---"),
    ('K', "-.-"),
    ('L', ".-.."),
    ('M', "--"),
    ('N', "-."),
    ('O', "---"),
    ('P', ".--."),
    ('Q', "--.-"),
    ('R', ".-."),
    ('S', "..."),
    ('T', "-"),
    ('U', "..-"),
    ('V', "...-"),
    ('W', ".--"),
    ('X', "-..-"),
    ('Y', "-.--"),
    ('Z', "--.."),
];

use arduino_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;

pub struct MorseWriter<P> {
    pin: P,
}

impl<P: OutputPin> MorseWriter<P> {
    pub fn new(pin: P) -> Self {
        MorseWriter { pin }
    }

    pub fn write(&mut self, c: char) {
        let morse = ALPHABET[(c as u8 - b'a') as usize].1;
        for b in morse.bytes() {
            match b {
                b'.' => {
                    self.pin.set_high();
                    arduino_hal::delay_ms(100);
                    self.pin.set_low();
                    arduino_hal::delay_ms(100);
                }
                b'-' => {
                    self.pin.set_high();
                    arduino_hal::delay_ms(300);
                    self.pin.set_low();
                    arduino_hal::delay_ms(100);
                }
                _ => {}
            }
        }
        arduino_hal::delay_ms(300);
    }
}