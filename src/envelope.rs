pub struct Envelope {
    attack: f32,  // seconds
    decay: f32,   // seconds
    release: f32, // seconds
    sample_rate: f32,
    pub sustain: f32, // amplitude
    // TODO: Maybe move these variables outside of the envelope?
    pub att_coef: f32,
    pub dec_coef: f32,
    pub rel_coef: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sample_rate: f32) -> Envelope {
        return Envelope {
            attack: attack,
            decay: decay,
            sustain: sustain,
            release: release, 
            sample_rate: sample_rate,
            att_coef: 1.0 / (sample_rate * attack),
            dec_coef: 1.0 / (sample_rate * decay),
            rel_coef: 1.0 / (sample_rate * release),
        }
    }
}