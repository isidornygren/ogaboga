extern crate noise;
extern crate ogaboga;
extern crate rand;

use ogaboga::waveforms::{sawtooth_wave, square_wave, triangle_wave};
use ogaboga::{Envelope, Voice, VoiceEvent, VoicePoolBuilder};
use rand::Rng;
use std::{thread, time};

fn main() {
    // Initiate the voice pool that we will initiate voices in
    let voice_pool = VoicePoolBuilder::new()
        .with_voice(Voice::new(
            &triangle_wave,
            Envelope::new(0.5, 0.5, 0.5, 0.5),
        ))
        .with_voice(Voice::new(
            &triangle_wave,
            Envelope::new(0.5, 0.5, 0.5, 0.5),
        ))
        .with_voice(Voice::new(
            &triangle_wave,
            Envelope::new(0.5, 0.5, 0.5, 0.5),
        ))
        .build();

    // loop {
    // let sleep_time = time::Duration::from_millis(1000);
    // thread::sleep(sleep_time);
    voice_pool.send(VoiceEvent::ChangeFreq(261.63), 0).unwrap();
    voice_pool.send(VoiceEvent::Start, 0).unwrap();
    voice_pool.send(VoiceEvent::ChangeFreq(329.63), 1).unwrap();
    voice_pool.send(VoiceEvent::Start, 1).unwrap();
    voice_pool.send(VoiceEvent::ChangeFreq(392.00), 2).unwrap();
    voice_pool.send(VoiceEvent::Start, 2).unwrap();

    // }
    loop {}
}
