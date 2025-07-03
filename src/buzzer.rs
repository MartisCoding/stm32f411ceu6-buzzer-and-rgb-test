use cortex_m::prelude::{_embedded_hal_Pwm, _embedded_hal_blocking_delay_DelayMs};
use heapless::Vec;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::{SimplePwm, PwmPin};
use embassy_time::Delay;

pub type Count = u8;
pub type Note = char;

pub struct Buzzer<'a> {
    pwm: SimplePwm<'a, peripherals::TIM1>,
    channel: Channel,
    note_sequence: Option<&'a Vec<(Hertz, Count), 64>>,
    tempo: Option<u32>,
}
impl<'a> Buzzer<'a> {
    pub fn new(timer: peripherals::TIM1, p: peripherals::PA9) -> Self {
        
        let pwm = SimplePwm::new(
            timer, None, Some(pin), None, None, Hertz(2000), CountingMode::EdgeAlignedUp
        );
        Buzzer {pwm, channel:Channel::Ch2, note_sequence: None, tempo: None}
    }

    pub fn set_tune(&mut self, tune: Vec<(Note, Count), 64>) {
        let mut frequencies: Vec<(Hertz, Count), 64> = Vec::new();
        for (note, count) in tune {
            let freq = match note {
                'c' => Hertz(261),
                'd' => Hertz(294),
                'e' => Hertz(329),
                'f' => Hertz(349),
                'g' => Hertz(392),
                'a' => Hertz(440),
                'b' => Hertz(493),
                'C' => Hertz(523), // высокая до
                '-' => Hertz(0),   // пауза
                _ => continue,
            };
            frequencies.push((freq, count)).unwrap();
        }
    }
    
    pub fn set_tempo(&mut self, bpm: u16) {
        self.tempo = Some(60_000 / (bpm as u32));
    }
    
    pub async fn buzz(&mut self) {
        let half_beat = self.tempo.unwrap() / 2;
        for (hertz, count) in self.note_sequence.unwrap() {
            self.pwm.set_frequency(*hertz);
            self.pwm.enable(self.channel);
            Delay.delay_ms(self.tempo.unwrap() * *count as u32);
            self.pwm.disable(self.channel);
            Delay.delay_ms(half_beat)
        }
    }
    pub fn buzz_test(&mut self) {
        self.pwm.set_frequency(Hertz(1000));
        self.pwm.enable(self.channel);
    }

}