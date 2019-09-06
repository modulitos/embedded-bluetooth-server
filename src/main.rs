#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::convert::TryFrom;

#[allow(unused_imports)]
use auxiliary::{entry, iprint, iprintln};
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = auxiliary::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    // iprintln!(&mut itm.stim[0], "starting!!!");
    loop {
        let over8_is_set = usart1.cr1.read().over8().bit_is_set();
        // iprintln!(&mut itm.stim[0], "over8: {}", over8_is_set);
        let brr_div = usart1.brr.read().bits();
        // iprintln!(&mut itm.stim[0], "brr_div: {:0x}", brr_div + 10);

        // iprintln!(&mut itm.stim[0], "outer loop - clearing buffer");
        buffer.clear();
        loop {
            // iprintln!(&mut itm.stim[0], "inside inner loop");

            while usart1.isr.read().rxne().bit_is_clear() {}
            // iprintln!(
            //     &mut itm.stim[0],
            //     "isr (interrupt and status register) is clear!"
            // );

            let byte = usart1.rdr.read().rdr().bits() as u8;
            // iprintln!(&mut itm.stim[0], "byte: {}", char::from(byte));

            if buffer.push(byte).is_err() {
                // buffer full
                // iprintln!(&mut itm.stim[0], "error pushing byte onto buffer");
                for byte in b"error: buffer full\n\r" {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }

            // Carriage return
            if byte == 13 {
                // iprintln!(&mut itm.stim[0], "byte is 13");
                // Respond
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }
        }
    }
    // loop {}
}
