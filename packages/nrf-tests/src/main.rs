#![no_std]
#![no_main]

use defmt_rtt as _;
use nrf_tests as _;
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    for x in 0..100 {
        defmt::info!("Hello, {:?}!", x);
    }
    nrf_tests::exit();
}
