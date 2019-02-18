const PI: f32 = 3.141592;
const PI_2: f32 = 2.0 * PI;

// Some waveform generators, you could just use f32::sin to get a normal sinusoid
pub fn square_wave(clock: f32) -> f32 {
    return (clock / PI_2).round() * 2.0 - 1.0;
}
pub fn sawtooth_wave(clock: f32) -> f32 {
    return clock / PI - 1.0;
}
pub fn triangle_wave(clock: f32) -> f32 {
    // Triangle wave is just a glorified sawtooth wave
    return (clock / PI - 1.0).abs();
}

#[cfg(test)]
mod tests {
    use super::*;
    const FLOAT_ERROR: f32 = 0.001;

    #[test]
    fn square_wave_is_periodic() {
        assert!(square_wave(0.0) - square_wave(PI_2 + 0.0001) < FLOAT_ERROR);
    }
    #[test]
    fn sawtooth_wave_is_periodic() {
        assert!(sawtooth_wave(0.0) - sawtooth_wave(PI_2 + 0.0001) < FLOAT_ERROR);
    }
    #[test]
    fn triangle_wave_is_periodic() {
        assert!(triangle_wave(0.0) - triangle_wave(PI_2 + 0.0001) < FLOAT_ERROR);
    }
}