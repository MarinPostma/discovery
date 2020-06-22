#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Leds, Delay};


fn print_state(leds: &mut Leds, states: &[u8; 8]) {
    for (i, v) in states.iter().enumerate() {
        match v {
            0 => leds[i].off(),
            1 => leds[i].on(),
            _ => unreachable!(),
        }
    }
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    }
    let half_period = 100_u16;
    let mut leds_state = [1, 0 ,0, 0, 0, 0 ,0 ,0];
    loop {
        print_state(&mut leds, &leds_state);
        leds_state = match leds_state {
            [1, 0 ,0, 0, 0, 0 ,0 ,0] => [1, 1 ,0, 0, 0, 0 ,0 ,0],
            [1, 1 ,0, 0, 0, 0 ,0 ,0] => [0, 1 ,0, 0, 0, 0 ,0 ,0],
            [0, 1 ,0, 0, 0, 0 ,0 ,0] => [0, 1 ,1, 0, 0, 0 ,0 ,0],
            [0, 1 ,1, 0, 0, 0 ,0 ,0] => [0, 0 ,1, 0, 0, 0 ,0 ,0],
            [0, 0 ,1, 0, 0, 0 ,0 ,0] => [0, 0 ,1, 1, 0, 0 ,0 ,0],
            [0, 0 ,1, 1, 0, 0 ,0 ,0] => [0, 0 ,0, 1, 0, 0 ,0 ,0],
            [0, 0 ,0, 1, 0, 0 ,0 ,0] => [0, 0 ,0, 1, 1, 0 ,0 ,0],
            [0, 0 ,0, 1, 1, 0 ,0 ,0] => [0, 0 ,0, 0, 1, 0 ,0 ,0],
            [0, 0 ,0, 0, 1, 0 ,0 ,0] => [0, 0 ,0, 0, 1, 1 ,0 ,0],
            [0, 0 ,0, 0, 1, 1 ,0 ,0] => [0, 0 ,0, 0, 0, 1 ,0 ,0],
            [0, 0 ,0, 0, 0, 1 ,0 ,0] => [0, 0 ,0, 0, 0, 1 ,1 ,0],
            [0, 0 ,0, 0, 0, 1 ,1 ,0] => [0, 0 ,0, 0, 0, 0 ,1 ,0],
            [0, 0 ,0, 0, 0, 0 ,1 ,0] => [0, 0 ,0, 0, 0, 0 ,1 ,1],
            [0, 0 ,0, 0, 0, 0 ,1 ,1] => [0, 0 ,0, 0, 0, 0 ,0 ,1],
            [0, 0 ,0, 0, 0, 0 ,0 ,1] => [1, 0 ,0, 0, 0, 0 ,0 ,1],
            [1, 0 ,0, 0, 0, 0 ,0 ,1] => [1, 0 ,0, 0, 0, 0 ,0 ,0],
            _ => unreachable!(),
        };
        delay.delay_ms(half_period);

    }
}
