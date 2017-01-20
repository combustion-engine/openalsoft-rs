extern crate openalsoft_sys as als;
#[macro_use]
extern crate trace_error;

extern crate pitch_calc as pitch;
#[macro_use]
extern crate openal;
extern crate nalgebra;
extern crate time_calc as time;
extern crate dsp;


use als::all::*;
use openal::*;

use std::thread::sleep;
use std::time::Duration;
//use std::sync::Arc;

//use nalgebra::*;
//use pitch::*;
//use time::*;
use dsp::*;
//use dsp::sample::ToFrameSliceMut;

type Output = f32;

type Phase = f64;
type Frequency = f64;
type Volume = f32;

const CHANNELS: usize = 2;

const A5_HZ: Frequency = 440.0;
const D5_HZ: Frequency = 587.33;
const F5_HZ: Frequency = 698.46;

fn main() {
    run().unwrap()
}

fn run() -> ALResult<()> {
    println!("Opening default device...");
    let device = try_rethrow!(ALDevice::open(None));

    println!("Creating OpenAL listener...");
    let listener = try_rethrow!(device.create_listener());

    try_rethrow!(listener.set_thread_context());

    let name = if try_rethrow!(device.extension_present("ALC_ENUMERATE_ALL_EXT")) {
        try_rethrow!(device.get_string(ALC_ALL_DEVICES_SPECIFIER))
    } else {
        try_rethrow!(device.get_string(ALC_DEVICE_SPECIFIER))
    };

    println!("Running {}", name);

    let device_srate = try_rethrow!(device.get_integer(ALC_FREQUENCY));

    println!("Device sample rate: {}", device_srate);

    let source = try_rethrow!(listener.new_3d_source());

    // Construct our dsp graph.
    let mut graph = Graph::new();

    // Construct our fancy Synth and add it to the graph!
    let synth = graph.add_node(DspNode::Synth);

    // Connect a few oscillators to the synth.
    graph.add_input(DspNode::Oscillator(0.0, A5_HZ, 0.2), synth);
    graph.add_input(DspNode::Oscillator(0.0, D5_HZ, 0.1), synth);
    graph.add_input(DspNode::Oscillator(0.0, F5_HZ, 0.15), synth);

    // Set the synth as the master node for the graph.
    graph.set_master(Some(synth));

    let mut sample = vec![[0.0; 2]; device_srate as usize * 10];

    graph.audio_requested(sample.as_mut_slice(), device_srate as f64);

    let buffer = try_rethrow!(ALBuffer::from_elements(&sample, ALFormat::common_stereo32f(device_srate as ALSampleRate)));

    try_rethrow!(source.set_looping(false));
    try_rethrow!(source.queue_buffers([buffer].iter().cloned()));
    try_rethrow!(source.play());

    sleep(Duration::from_secs(10));

    try_rethrow!(source.stop());

    Ok(())
}

/// Our type for which we will implement the `Dsp` trait.
#[derive(Debug)]
enum DspNode {
    /// Synth will be our demonstration of a master GraphNode.
    Synth,
    /// Oscillator will be our generator type of node, meaning that we will override
    /// the way it provides audio via its `audio_requested` method.
    Oscillator(Phase, Frequency, Volume),
}

impl Node<[Output; CHANNELS]> for DspNode {
    /// Here we'll override the audio_requested method and generate a sine wave.
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        match *self {
            DspNode::Synth => (),
            DspNode::Oscillator(ref mut phase, frequency, volume) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let val = sine_wave(*phase, volume);
                    *phase += frequency / sample_hz;
                    Frame::from_fn(|_| val)
                });
            },
        }
    }
}

/// Return a sine wave for the given phase.
fn sine_wave<S: Sample>(phase: Phase, volume: Volume) -> S
                        where S: Sample + FromSample<f32>,
{
    use std::f64::consts::PI;
    ((phase * PI * 2.0).sin() as f32 * volume).to_sample::<S>()
}
