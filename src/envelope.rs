pub struct Envelope {
    pub attack: f32,  // seconds
    pub decay: f32,   // seconds
    pub release: f32, // seconds
    pub sustain: f32, // amplitude
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Envelope {
        return Envelope {
            attack: attack,
            decay: decay,
            sustain: sustain,
            release: release, 
        }
    }
}