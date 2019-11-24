use crate::wave_generator::WaveBox;
use rand::Rng;

const PI_2: f32 = 2.0 * std::f32::consts::PI;

// Some waveform generators, you could just use f32::sin to get a normal sinusoid
#[inline]
pub fn square_wave(clock: f32) -> f32 {
    return (clock / PI_2).round() * 2.0 - 1.0;
}
#[inline]
pub fn sawtooth_wave(clock: f32) -> f32 {
    return clock / std::f32::consts::PI - 1.0;
}
#[inline]
pub fn triangle_wave(clock: f32) -> f32 {
    // Triangle wave is just a glorified sawtooth wave
    return (clock / std::f32::consts::PI - 1.0).abs() * 2.0 - 1.0;
}

#[inline]
pub fn white_noise(_clock: f32) -> f32 {
    // Triangle wave is just a glorified sawtooth wave
    return rand::thread_rng().gen::<f32>() * 2.0 - 1.0;
}

#[inline]
pub fn freq_noise(multiplier: f32) -> WaveBox {
    let mut previous_value = 0.0;
    Box::new(move |x: f32| {
        // println!("In freq noise: {}", previous_value);
        let new_value = (previous_value * (1.0 - multiplier)) + (white_noise(x) * multiplier);
        previous_value = new_value;
        return new_value;
    })
}

#[inline]
pub fn one_bit_noise(_clock: f32) -> f32 {
    // Triangle wave is just a glorified sawtooth wave
    return (rand::thread_rng().gen::<f32>() * 2.0 - 1.0).round();
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
