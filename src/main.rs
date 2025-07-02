#![no_std]
#![no_main]

mod fmt;
mod buzzer;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_stm32::gpio::{ Level, Speed, Output,};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};
use heapless::Vec;
use embassy_executor::Spawner;
use embassy_time::Delay;
use fmt::info;

const PERIOD: u32 = 125;
const MELODY: [(buzzer::Note, buzzer::Count); 32] = [
    ('e',1),('e',1),('f',1),('g',1),('g',1),('f',1),('e',1),('d',1),
    ('c',1),('c',1),('d',1),('e',1),('e',2),('-',1),('e',1),('e',1),
    ('f',1),('g',1),('g',1),('f',1),('e',1),('d',1),('c',1),('c',1),
    ('d',1),('e',1),('d',2),('-',1),('d',1),('d',1),('e',1),('c',1),
];
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    led.set_high();
    Delay.delay_ms(500_u32);
    let tune: Vec<(buzzer::Note, buzzer::Count), 64> = Vec::from_slice(&MELODY).unwrap();
    let mut buzzer = buzzer::Buzzer::new(p.TIM1, p.PA9);
    buzzer.set_tempo(150);
    buzzer.set_tune(tune);
    
    loop {
        led.set_high();
        buzzer.buzz().await;
        Delay.delay_ms(1000_u32);
        led.set_low();
        Delay.delay_ms(1000_u32);
    }
}


