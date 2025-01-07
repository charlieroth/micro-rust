#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
extern crate cortex_m;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world");
    loop {
        rprintln!("Echo");
    }
}
