const A4: f64 = 440.0;

#[derive(Clone)]
pub enum Note {
   A = 0,
   A_,
   B,
   C,
   C_,
   D,
   D_,
   E,
   F,
   F_,
   G,
   G_,
}

impl Note {
   #[must_use]
   #[inline]
   pub fn get_freq(&self, octave: u16) -> f64 {
      // const key = note
      let new_note = if (self.clone() as u32) < 3 {
         (self.clone() as i64) + 12 + ((octave as i64 - 1) * 12) + 1
      } else {
         (self.clone() as i64) + ((octave as i64 - 1) * 12) + 1
      };

      // Return frequency of note
      return A4 * ((new_note as f64 - 49.0) / 12.0).exp2();
   }
}
