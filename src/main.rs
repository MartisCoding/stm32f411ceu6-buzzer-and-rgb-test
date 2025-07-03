#![no_std]
#![no_main]
mod fmt;
use cortex_m::prelude::{_embedded_hal_Pwm, _embedded_hal_blocking_delay_DelayMs};
use embassy_stm32::gpio::{Level, Speed, Output, OutputType};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};
//use heapless::Vec;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::time::hz;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_time::Delay;
use fmt::info;


use embassy_stm32::timer::simple_pwm::{SimplePwm, PwmPin};


//const PERIOD: u32 = 125;
///const MELODY: [(buzzer::Note, buzzer::Count); 32] = [
//    ('e',1),('e',1),('f',1),('g',1),('g',1),('f',1),('e',1),('d',1),
//    ('c',1),('c',1),('d',1),('e',1),('e',2),('-',1),('e',1),('e',1),
//    ('f',1),('g',1),('g',1),('f',1),('e',1),('d',1),('c',1),('c',1),
//    ('d',1),('e',1),('d',2),('-',1),('d',1),('d',1),('e',1),('c',1),
//];
//#[embassy_executor::main]
//async fn main(_spawner: Spawner) {
//    info!("Entry!");
//    let p = embassy_stm32::init(Default::default());
//    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
//    led.set_high();
//    info!("Led check!");
//    Delay.delay_ms(500_u32);
//    let tune: Vec<(buzzer::Note, buzzer::Count), 64> = Vec::from_slice(&MELODY).unwrap();
//    let mut buzzer = buzzer::Buzzer::new(p.TIM1, p.PA9);
//    buzzer.buzz_test();
//    info!("Buzz test!");
//    Delay.delay_ms(20000_u32);
//
//    buzzer.set_tempo(150);
//    buzzer.set_tune(tune);
//    info!("Main loop!");
//    loop {
//        led.set_high();
//        buzzer.buzz().await;
//        Delay.delay_ms(1000_u32);
//        led.set_low();
//        Delay.delay_ms(1000_u32);
//    }
//}


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let buzz_pin = PwmPin::new_ch2(p.PA9, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM1, None, Some(buzz_pin), None, None, hz(2000), CountingMode::EdgeAlignedDown);

    let max_duty = pwm.get_max_duty();
    pwm.set_duty(Channel::Ch2, max_duty / 2);

    let tempo = 300_u32;

    let tones = [
        ('c', hz(261)),
        ('d', hz(294)),
        ('e', hz(329)),
        ('f', hz(349)),
        ('g', hz(392)),
        ('a', hz(440)),
        ('b', hz(493)),
    ];

    let tune = [
        ('c', 1),
        ('c', 1),
        ('g', 1),
        ('g', 1),
        ('a', 1),
        ('a', 1),
        ('g', 2),
        ('f', 1),
        ('f', 1),
        ('e', 1),
        ('e', 1),
        ('d', 1),
        ('d', 1),
        ('c', 2),
        (' ', 4),
    ];

    loop {
        for note in tune {
            for tone in tones {
                if tone.0 == note.0 {
                    pwm.set_frequency(tone.1);
                    pwm.enable(Channel::Ch2);
                    Delay.delay_ms(note.1 as u32 * tempo);
                } else if tone.0 == ' ' {
                    pwm.disable(Channel::Ch2);
                    Delay.delay_ms(tempo);
                }
            }
            pwm.disable(Channel::Ch2);
            Delay.delay_ms(tempo / 2);
        }
    }
}

