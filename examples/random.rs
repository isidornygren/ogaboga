extern crate noise;
extern crate ogaboga;
extern crate rand;

use ogaboga::{Envelope, Voice, VoiceEvent, VoicePoolBuilder};
use rand::Rng;
use std::{thread, time};

fn main() {
    // Initiate the voice pool that we will initiate voices in
    let voice_pool = VoicePoolBuilder::new()
        .with_voice(Voice::new(&f32::sin, Envelope::new(0.5, 0.5, 0.5, 0.5)))
        .build();

    loop {
        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
        voice_pool
            .send(
                VoiceEvent::ChangeFreq(440.0 + rand::thread_rng().gen::<f32>() * 220.0),
                0,
            )
            .unwrap();
        voice_pool.send(VoiceEvent::Pulse, 0).unwrap();
    }
}
