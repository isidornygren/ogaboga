extern crate ogaboga;
extern crate rand;

use crate::ogaboga::{
    scale::Db_MINOR,
    sequencer::{
        generator::{BeatGenerator, SequenceGenerator, TuneGenerator},
        SequenceStep, SequencerBuilder,
    },
    waveforms::{freq_noise, square_wave, triangle_wave},
    Envelope, PolyRhythmController, Voice, VoiceEvent, VoicePool,
};

fn main() {
    // Initiate the voice pool that we will initiate voices in
    let mut voice_pool = VoicePool::new();
    let base_index = voice_pool.add_voice(Voice::new(
        Box::new(triangle_wave),
        Envelope::new(0.01, 0.1, 0.1, 0.3),
    ));
    let hihat_index = voice_pool.add_voice(Voice::new(
        freq_noise(0.1),
        Envelope::new(0.001, 0.1, 0.1, 0.25),
    ));
    let main_index = voice_pool.add_voice(Voice::new(
        Box::new(triangle_wave),
        Envelope::new(0.1, 0.3, 0.6, 0.4),
    ));

    voice_pool
        .send(VoiceEvent::ChangeFreq(20.0), base_index)
        .unwrap();
    voice_pool
        .send(VoiceEvent::ChangeFreq(1000.0), hihat_index)
        .unwrap();
    voice_pool
        .send(VoiceEvent::ChangeAmp(0.25), hihat_index)
        .unwrap();
    voice_pool
        .send(VoiceEvent::ChangeAmp(0.2), main_index)
        .unwrap();

    //  let sequencer_builder =
    //      SequencerBuilder::new(Box::new(PolyRhythmController::new(&[500 / 2, 750 / 2])));
    let sequencer_builder =
        SequencerBuilder::new(Box::new(PolyRhythmController::new(&[1200, 800, 1100])));

    let base_drum_generator = BeatGenerator::new()
        .period_fraction(1.0 / 2.0)
        .chance_range(0.5, 1.0);
    // let base_drum_generator = TuneGenerator::new(Db_MINOR.to_vec(), 1);

    let high_hat_generator = BeatGenerator::new()
        .period_fraction(1.0 / 2.0)
        .chance_range(0.0, 0.5)
        .period_offset(std::f32::consts::PI);

    let tune_generator = TuneGenerator::new(Db_MINOR.to_vec(), 2);

    let base_drum_sequence = base_drum_generator.generate(8);
    let high_hat_sequence = high_hat_generator.generate(8);
    let tune_sequence = tune_generator.generate(6);

    let mut sequencer = sequencer_builder
        .add_sequence(base_drum_sequence.clone())
        .add_sequence(high_hat_sequence.clone())
        .add_sequence(tune_sequence.clone())
        .build();

    sequencer.run_then(
        |index, step| match step {
            Some(SequenceStep::Beat) => {
                voice_pool.send(VoiceEvent::Pulse, index).unwrap();
            }
            Some(SequenceStep::Freq(freq)) => {
                voice_pool
                    .send(VoiceEvent::PulseFreq(*freq), index)
                    .unwrap();
            }
            _ => {}
        },
        |index, sequence| {
            if index == base_index {
                // only mutating the base value a little bit every time
                return base_drum_generator.mutate(&base_drum_sequence, 0.2);
            } else if index == hihat_index {
                // continously mutating the value
                return high_hat_generator.mutate(sequence, 0.5);
            // } else if index == main_index {
            //    // only mutating the base value a little bit every time
            //    return tune_generator.mutate(&tune_sequence, 0.2);
            } else {
                return sequence.clone();
            }
        },
    );
}
