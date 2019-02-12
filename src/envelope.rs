pub struct Envelope {
    pub attack: f32,  // seconds
    pub decay: f32,   // amplitude
    pub sustain: f32, // seconds
    pub release: f32, // seconds
}

impl Envelope {
    pub fn get_amplitude(&self, seconds: f32) -> f32 {
        if seconds <= self.attack{
            // attack
            return seconds / self.attack;
        }else if seconds < self.attack + self.decay{
            // decay
            return ((self.decay - (seconds - self.attack)) / self.decay) * (1.0 - self.sustain) + self.sustain;

        }else if seconds < self.attack + self.decay + self.release{
            // here is where sustain should be if there was one 
            // release
            return self.sustain * (1.0 - (seconds - self.attack - self.decay) / self.release);
        }
        return 0.0;
    }
}
