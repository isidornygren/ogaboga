# OgaBoga
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/isidornygren/ogaboga/blob/master/LICENSE)

Ogaboga is a syntheziser library written in rust, it's very dumb and you shouldn't really use it if you don't like to do fun stuff that's not terribly optimized.

It consists of a pulse modulator that takes an envelope and a waveform and can ouput a pulse based on that envelope.

The waveform is simply a function that takes a _clock_ that will be the _deltatime_ modulus the common period `2*PI` and outputs a waveform accordingly, some examples like square, triangle and sawtooth are included.

A single voice will run in its own thread, so you'll need a voice pool to handle the communication between threads. A simple program that will send a pulse of different frequency every 1 second follows below.

## Examples
See the examples/ directory in the source. 

To run the examples, just check out the source and execute cargo run --example in the root directory:
```
$ git clone https://github.com/isidornygren/ogaboga.git
$ cd ogaboga
$ cargo run --example random
```
### Basic project example

```rust
fn main() {
    let device = cpal::default_output_device().unwrap();
    let format = device.default_output_format().unwrap();

    let mut voice_pool = VoicePool::new();
    voice_pool.add_voice(Voice::new(format.sample_rate.0, 
        Envelope::new(0.5,0.5,0.5,0.5), &f32::sin));

    loop {
        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
        voice_pool.send(
            VoiceEvent::ChangeFreq(440.0 + 
                rand::thread_rng().gen::<f32>() * 220.0), 
                0).unwrap();

        voice_pool.send(VoiceEvent::Pulse, 0).unwrap();
    };
}
```

## TODO
- Remove sample rate dependency on voice creation, as the format is deduced during thread creation.
- Tests
- Integrate Travis
- Periodic 1D noice function
- Rethink abstractions and fix library linking
- Examples