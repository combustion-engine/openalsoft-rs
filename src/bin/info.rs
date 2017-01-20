extern crate openal;
extern crate openalsoft_sys as als;
#[macro_use]
extern crate trace_error;

use std::sync::Arc;

use openal::*;
use als::all::*;
use als::ext::*;
use als::efx::*;

fn main() {
    run().unwrap();
}

fn run() -> ALResult<()> {
    let enumerate_all = try_rethrow!(NULL_DEVICE.extension_present(ALC_ENUMERATE_ALL_EXT_NAME));

    let playback_devices = if enumerate_all {
        try_rethrow!(NULL_DEVICE.get_multistring(ALC_ALL_DEVICES_SPECIFIER))
    } else {
        try_rethrow!(NULL_DEVICE.get_multistring(ALC_DEVICE_SPECIFIER))
    };

    println!("Available playback devices:");
    for device in &playback_devices {
        println!("  {}", device);
    }

    println!("Available capture devices:");
    for device in &try_rethrow!(NULL_DEVICE.get_multistring(ALC_CAPTURE_DEVICE_SPECIFIER)) {
        println!("  {}", device);
    }

    let default_devices = if enumerate_all {
        try_rethrow!(NULL_DEVICE.get_multistring(ALC_DEFAULT_ALL_DEVICES_SPECIFIER))
    } else {
        try_rethrow!(NULL_DEVICE.get_multistring(ALC_DEFAULT_DEVICE_SPECIFIER))
    };

    println!("Default playback devices:");
    for device in &default_devices {
        println!("  {}", device);
    }

    try_rethrow!(alc_info(NULL_DEVICE.clone()));

    let device = try_rethrow!(ALDevice::open(None));

    try_rethrow!(alc_info(device.clone()));
    try_rethrow!(hrtf_info(device.clone()));

    let _context = try_rethrow!(device.create_context());

    try_rethrow!(al_info());
    try_rethrow!(efx_info(device.clone()));

    Ok(())
}

fn alc_info(device: Arc<ALDevice>) -> ALResult<()> {
    if device != *NULL_DEVICE {
        println!("Info for device: {}", try_rethrow!(device.name()));
    } else {
        println!("Generic info:");
    }

    let major = try_rethrow!(device.get_integer(ALC_MAJOR_VERSION));
    let minor = try_rethrow!(device.get_integer(ALC_MINOR_VERSION));

    println!("ALC Version: {}.{}", major, minor);

    if device != *NULL_DEVICE {
        println!("Extensions:");
        for ext in try_rethrow!(device.get_string(ALC_EXTENSIONS)).split_whitespace() {
            println!("  {}", ext);
        }
    }

    Ok(())
}

fn hrtf_info(device: Arc<ALDevice>) -> ALResult<()> {
    if try_rethrow!(device.extension_present(ALC_SOFT_HRTF_NAME)) {
        let num_hrtfs = try_rethrow!(device.get_integer(ALC_NUM_HRTF_SPECIFIERS_SOFT));

        if num_hrtfs == 0 {
            println!("No HRTFs found");
        } else {
            for i in 0..num_hrtfs {
                println!("  {}", try_rethrow!(device.get_stringi(ALC_HRTF_SPECIFIER_SOFT, i)));
            }
        }
    } else {
        println!("HRTF extension not available");
    }

    Ok(())
}

fn al_info() -> ALResult<()> {
    println!("OpenAL vendor string:   {}", try_rethrow!(ALState::get_string(AL_VENDOR)));
    println!("OpenAL renderer string: {}", try_rethrow!(ALState::get_string(AL_RENDERER)));
    println!("OpenAL version string:  {}", try_rethrow!(ALState::get_string(AL_VERSION)));
    println!("OpenAL extensions:");

    for ext in try_rethrow!(ALState::get_string(AL_EXTENSIONS)).split_whitespace() {
        println!("  {}", ext);
    }

    Ok(())
}

fn efx_info(device: Arc<ALDevice>) -> ALResult<()> {
    if try_rethrow!(device.extension_present(ALC_EXT_EFX_NAME)) {
        let major = try_rethrow!(device.get_integer(ALC_EFX_MAJOR_VERSION));
        let minor = try_rethrow!(device.get_integer(ALC_EFX_MINOR_VERSION));

        println!("EFX version: {}.{}", major, minor);

        let sends = try_rethrow!(device.get_integer(ALC_MAX_AUXILIARY_SENDS));

        println!("Max auxiliary sends: {}", sends);

        let filters = AL_EFX_FILTERS.iter().zip(AL_EFX_FILTER_NAMES);
        let effects = AL_EFX_EFFECTS.iter().zip(AL_EFX_EFFECT_NAMES);
        let dedicated_effects = AL_EFX_DEDICATED_EFFECTS.iter().zip(AL_EFX_DEDICATED_EFFECT_NAMES);

        println!("Supported effects:");

        let mut unsupported = Vec::new();

        for (effect, name) in filters.chain(effects).chain(dedicated_effects) {
            if ALState::get_enum(effect).is_ok() {
                println!("  {}", name);
            } else {
                unsupported.push(name);
            }
        }

        println!("Unsupported effects: ");

        for name in &unsupported {
            println!("  {}", name);
        }
    } else {
        println!("EFX not available");
    }

    Ok(())
}