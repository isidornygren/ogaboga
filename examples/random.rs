extern crate ogaboga;
extern crate rand;

use ogaboga::{waveforms::triangle_wave, Envelope, Voice, VoiceEvent, VoicePool};

fn main() {
   // Initiate the voice pool that we will initiate voices in
   let mut voice_pool = VoicePool::new();

   for _ in 0..3 {
      voice_pool.add_voice(Voice::new(
         Box::new(triangle_wave),
         Envelope::new(0.5, 0.5, 0.5, 0.5),
      ));
   }

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
