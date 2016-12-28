extern crate openalsoft_sys as als;
extern crate pitch_calc as pitch;
#[macro_use]
extern crate openal;
extern crate nalgebra;
extern crate time_calc as time;

use als::all::*;
use openal::*;

use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

use pitch::*;
use nalgebra::*;
use time::*;

fn sine_wave(mut data: &mut [ALfloat], g: ALdouble, srate: ALuint, freq: ALuint) {
    for (i, d) in data.iter_mut().enumerate() {
        let mut v = 0.0;

        let smps_per_cycle = srate as f64 / freq as f64;

        v += (i as f64 / smps_per_cycle * 2.0 * ::std::f64::consts::PI).sin();

        if v > 1.0 {
            v = 1.0;
        } else if v < -1.0 {
            v = -1.0;
        }

        *d += (v * g) as ALfloat;
    }
}

unsafe fn create_wave(freq: ALuint, srate: ALuint, time: f32) -> ALResult<Arc<ALBuffer>> {
    let mut data = vec![0.0 as ALfloat; (srate as f32 * time) as usize];

    sine_wave(&mut data, 1.0, srate, freq);

    let mut buffer = ALBuffer::new()?;

    buffer.buffer_elements(&data, ALFormat::StereoFloat32(srate))?;

    Ok(buffer)
}

unsafe fn run() -> ALResult<()> {
    println!("Opening default device...");
    let device = ALDevice::open(None)?;

    println!("Creating OpenAL context...");
    let ctx = device.create_context()?;

    let listener = ALListener::new(device.clone(), ctx.clone());

    listener.set_thread_context()?;

    let name = if device.extension_present("ALC_ENUMERATE_ALL_EXT")? {
        device.get_string(ALC_ALL_DEVICES_SPECIFIER)
    } else {
        device.get_string(ALC_DEVICE_SPECIFIER)
    }?;

    println!("Running {}", name);

    let mut device_srate: ALint = 0;

    alcGetIntegerv(device.raw(), ALC_FREQUENCY, 1, &mut device_srate as *mut _);

    check_alc_errors!();

    println!("Device sample rate: {}", device_srate);

    let mut source = ALSource3D::new()?;

    let notes = {
        use Letter::*;

        [
            (F, 4, 1.0),
            (D, 4, 1.0),
            (C, 4, 1.0),
            (Bb, 3, 1.0),
            (C, 4, 1.0),
            (D, 4, 1.0),
            (F, 4, 1.0),
            (D, 4, 1.0),
            (C, 4, 1.0),
            (Bb, 3, 1.0),
            (C, 4, 0.5),
            (D, 4, 0.5),
            (C, 4, 0.5),
            (D, 4, 0.5),
            (F, 4, 1.0),
            (D, 4, 1.0),
            (F, 4, 1.0),
            (G, 4, 1.0),
            (D, 4, 1.0),
            (G, 4, 1.0),
            (F, 4, 1.0),
            (D, 4, 1.0),
            (C, 4, 1.0),
            (Bb, 3, 1.0),
        ]
    };

    let speed: f32 = 1.1;

    let buffers = notes.iter()
                       .map(|los| (hz_from_letter_octave(los.0, los.1 - 1), los.2 / speed))
                       .map(|fs| create_wave(fs.0 as u32, device_srate as ALSampleRate, fs.1).unwrap());


    source.set_position(Point3::new(0.0, 0.0, 0.0))?;
    source.queue_buffers(buffers)?;
    source.set_looping(false)?;
    source.play()?;

    sleep(Duration::from_millis((notes.len() as f64 * speed as f64 * 500.0) as u64));

    source.stop()?;

    Ok(())
}

fn main() {
    unsafe {
        run().unwrap();
    }
}