#[derive(Debug, Copy, Clone)]
pub struct Envelope {
   pub attack: f32,  // seconds
   pub decay: f32,   // seconds
   pub sustain: f32, // amplitude
   pub release: f32, // seconds
}

impl Envelope {
   #[must_use]
   #[inline]
   pub const fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
      return Self {
         attack,
         decay,
         sustain,
         release,
      };
   }
}
