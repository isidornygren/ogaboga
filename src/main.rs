extern crate cpal;
extern crate rand;
extern crate noise;

use std::{thread, time};
use rand::Rng;

mod wave_generator;
mod pulse_modulator;
mod envelope;
mod voice;
mod voice_pool;

use self::wave_generator::{sawtooth_wave, square_wave, triangle_wave};
use self::envelope::Envelope;
use self::voice::Voice;
use self::voice_pool::{VoicePool, VoiceEvent};

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");

    // let voices = vec![voice];
    let mut voice_pool = VoicePool::new();
    voice_pool.add_voice(Voice::new(format.sample_rate.0, Envelope::new(0.5,0.5,0.5,0.5), &sawtooth_wave));
    voice_pool.add_voice(Voice::new(format.sample_rate.0, Envelope::new(0.25,0.5,0.8,0.1), &square_wave));
    voice_pool.add_voice(Voice::new(format.sample_rate.0, Envelope::new(0.1,1.0,0.75,0.25), &triangle_wave));

    loop {
        // run this loop so the program won't close
        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
        voice_pool.send(VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0), 0);
        voice_pool.send(VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0), 1);
        voice_pool.send(VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0), 2);

        voice_pool.send(VoiceEvent::Pulse, 0);
        voice_pool.send(VoiceEvent::Pulse, 1);
        voice_pool.send(VoiceEvent::Pulse, 2);
    };
}
