extern crate cpal;
extern crate rand;
extern crate noise;

extern crate ogaboga;

use std::{thread, time};
use rand::Rng;
use ogaboga::{Envelope, Voice, VoicePool, VoiceEvent};
use ogaboga::waveforms::{sawtooth_wave, square_wave, triangle_wave};

pub fn main() {
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
        voice_pool.send(VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0), 0).unwrap();
        voice_pool.send(VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0), 1).unwrap();
        voice_pool.send(VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0), 2).unwrap();

        voice_pool.send(VoiceEvent::Pulse, 0).unwrap();
        voice_pool.send(VoiceEvent::Pulse, 1).unwrap();
        voice_pool.send(VoiceEvent::Pulse, 2).unwrap();
    };
}