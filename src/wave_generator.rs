const PI_2: f32 = 2.0 * std::f32::consts::PI;

pub type WaveForm = &'static (dyn FnMut(f32) -> f32 + Sync);
pub type WaveBox = Box<dyn FnMut(f32) -> f32 + Sync + Send>;

pub struct WaveGenerator {
   sample_rate: u32,
   waveform: WaveBox,
   step_size: f32,
   current_step: f32,
}

impl WaveGenerator {
   #[inline]
   pub fn new(sample_rate: u32, freq: f32, waveform: WaveBox) -> Self {
      let mut wave_struct = Self {
         sample_rate,
         waveform,
         step_size: 0.0,
         current_step: 0.0,
      };
      wave_struct.set_freq(freq);
      return wave_struct;
   }

   pub fn next(&mut self) -> f32 {
      // Will create a period between 0 and and 2*PI
      self.current_step = (self.current_step + self.step_size) % PI_2;
      return self.gen_sample(self.current_step);
   }

   #[inline]
   pub fn set_waveform(&mut self, waveform: WaveBox) {
      self.waveform = waveform;
   }

   #[inline]
   pub fn set_freq(&mut self, freq: f32) {
      self.step_size = (PI_2 * freq) / self.sample_rate as f32;
   }

   #[inline]
   pub fn set_step_size_frac(&mut self, fraction: f32) {
      assert!(fraction > 0.0, "The fraction needs to be a positive value");
      self.step_size = (PI_2) / fraction;
   }

   #[inline]
   fn gen_sample(&mut self, position: f32) -> f32 {
      assert!(
         position >= 0.0,
         "Position needs to be a positive value: '{:?}'",
         position
      );
      return (self.waveform)(position % PI_2);
   }

   #[inline]
   pub fn gen_full_set(&mut self, samples: u16) -> Vec<f32> {
      let mut sample_set = vec![];
      for n in 0..samples {
         sample_set.push(self.gen_sample(f32::from(n) / f32::from(samples) * PI_2));
      }
      return sample_set;
   }
}
