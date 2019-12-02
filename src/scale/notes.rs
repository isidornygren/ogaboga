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
	pub fn get_freq(&self, octave: u16) -> f64 {
		// const key = note
		let mut new_note: i64 = 0;
		if (self.clone() as u32) < 3 {
			new_note = (self.clone() as i64) + 12 + ((octave as i64 - 1) * 12) + 1;
		} else {
			new_note = (self.clone() as i64) + ((octave as i64 - 1) * 12) + 1;
		}

		// Return frequency of note
		return A4 * 2.0_f64.powf((new_note as f64 - 49.0) / 12.0);
	}
}
