extern crate cpal;
extern crate rand;
extern crate noise;

mod wave_generator;
mod pulse_modulator;
mod envelope;

use rand::prelude::*;
use noise::{NoiseFn, Perlin};
use self::wave_generator::{WaveStruct, SineWave};
use self::pulse_modulator::PulseModulator;
use self::envelope::Envelope;

// use wave_generator::SineWave;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    // let sample_clock = 0f32;
    // let perlin = Perlin::new();

    let mut wave_struct = WaveStruct::new(sample_rate, 880.0, SineWave {});
    let mut pulse_modulator = PulseModulator::new(
        sample_rate,
        Envelope {
            attack: 0.25,
            decay: 1.0,
            sustain: 0.8,
            release: 0.1,
        },
        wave_struct
    );

    // let mut next_value = || {
        // sample_clock = (sample_clock + 1.0) % sample_rate;
        // ordinary boring sine wave
        // (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
        // square wave
        // (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin().signum()
        // easy peasy triangle wave
        // (((sample_clock * 440.0/ sample_rate * 2.0) % 4.0) - 2.0).abs() - 1.0
        // omg is this a sawtooth wave?!
        // (((sample_clock * 440.0/ sample_rate * 2.0) % 4.0) - 2.0) % 2.0 - 1.0
        // what the hell something has gone wrong what is this abomination?!?!?!
        // rand::thread_rng().gen::<f32>() * 2.0 - 1.0
        // perlin.get([(sample_clock * 0.0099)as f64, 1.0]) as f32
    // };


    event_loop.run(move |_, data| {
        // wave_struct.change_freq(440.0 + wave_struct.current_clock);
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    // let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    let value = ((pulse_modulator.next() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (pulse_modulator.next() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = pulse_modulator.next();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
});
}
