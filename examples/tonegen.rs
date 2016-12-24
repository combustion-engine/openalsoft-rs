extern crate openalsoft_sys as als;
extern crate pitch_calc as pitch;
#[macro_use]
extern crate openal;
extern crate nalgebra;

use als::all::*;
use openal::*;

use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

use pitch::*;
use nalgebra::*;

fn binaural_beat(mut data: &mut [ALfloat], g: ALdouble, srate: ALuint, freq: ALuint) {
    for (i, d) in data.iter_mut().enumerate() {
        let mut v = 0.0;

        let smps_per_cycle = srate as f64 / (freq) as f64;

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
    let mut data = vec![0.0 as ALfloat; (srate as f32 * time) as usize / 2];

    binaural_beat(&mut data, 1.0, srate, freq);

    let mut buffer: ALBuffer = ALBuffer::new()?;

    buffer.buffer_elements(&data, ALFormat::MonoFloat32(srate))?;

    Ok(Arc::new(buffer))
}

unsafe fn run() -> ALResult<()> {
    println!("Opening default device...");
    let mut device: ALDevice = ALDevice::open()?;

    println!("Creating OpenAL context...");
    let mut ctx: ALContext = device.create_context()?;

    ctx.make_current()?;

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

    let mut source: ALSource3D = ALSource3D::new()?;

    let spider = [
        Letter::G,
        Letter::C,
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::E,
        Letter::E,
        Letter::D,
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::C,
        Letter::E,
        Letter::E,
        Letter::F,
        Letter::G,
    ];

    let speed: f32 = 0.5;

    let spider_buffers = spider.iter()
                               .map(|l| hz_from_letter_octave(*l, 4))
                               .map(|f| create_wave(f as u32, device_srate as ALSampleRate, speed).unwrap());


    source.set_position(Point3::new(0.0, 0.0, 0.0))?;
    source.queue_buffers(spider_buffers)?;
    source.set_looping(true)?;
    source.play()?;

    sleep(Duration::from_millis((spider.len() as f64 * speed as f64 * 5000.0) as u64));

    source.stop()?;

    Ok(())
}

fn main() {
    unsafe {
        run().unwrap();
    }
}