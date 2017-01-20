//! Effects
//!
//! Every effect and filter listed here has a checks and bounds feature that can be used either via the setter methods on every structure,
//! via the `new_checked` constructor function, or after creation and manual property setting via the `check` member function function.
//!
//! All checks and bounds have been taken from http://kcat.strangesoft.net/misc-downloads/Effects%20Extension%20Guide.pdf
use als::efx::EFXEAXREVERBPROPERTIES;
use als::all::*;

use super::error::*;

use nalgebra::*;

/// Types of effects supported by OpenAL Soft
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEffectType {
    Null,
    Reverb(ALEfxReverbProperties),
    EAXReverb(ALEfxEAXReverbProperties),
    Chorus(ALEfxChorusProperties),
    Distortion(ALEfxDistortionProperties),
    Echo(ALEfxEchoProperties),
    Flanger(ALEfxFlangerProperties),
    FrequencyShifter(ALEfxFrequencyShifterProperties),
    RingModulator(ALEfxRingModulatorProperties),
    Autowah(ALEfxAutowahProperties),
    Compressor(ALEfxCompressorProperties),
    Equalizer(ALEfxEqualizerProperties),
}

/// Types of filters supported by OpenAL Soft
#[derive(Debug, Serialize, Deserialize)]
pub enum ALFilterType {
    Null,
    Lowpass(ALEfxLowpassFilterProperties),
    Highpass(ALEfxHighpassFilterProperties),
    Bandpass(ALEfxBandpassFilterProperties),
}

// Have I ever mentioned Rust macros are amazing?
macro_rules! efx_struct {
    {$(#[$struct_attr:meta])* pub struct $name:ident { $( $(#[$field_attr:meta])* pub $field:ident: $t:ty = { $default:expr } $(in [$min:expr, $max:expr])* $(with $check:expr)*),*, } } => {
        $(#[$struct_attr])*
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            $(
                $(#[$field_attr])*
                pub $field: $t,
            )*
        }

        impl $name {
            /// Create a new structure from individual fields without checking bounds
            pub fn new($($field: $t),*) -> $name {
                $name {
                    $(
                        $field: $field,
                    )*
                }
            }

            /// Checks bounds for all fields when creating the structure
            pub fn new_checked($($field: $t),*) -> ALResult<$name> {
                Ok($name {
                    $(
                        $field: {
                            $(
                                if $field < $min || $max < $field {
                                    throw!(ALError::InvalidValue);
                                }
                            )*

                            $(
                                if !$check(&$field) {
                                    throw!(ALError::InvalidValue);
                                }
                            )*

                            $field
                        },
                    )*
                })
            }

            /// Checks that all fields are within their checks and bounds.
            pub fn check(&self) -> ALResult<()> {
                $(
                    $(
                        if self.$field < $min || $max < self.$field {
                            throw!(ALError::InvalidValue);
                        }
                    )*

                    $(
                        if !$check(&self.$field) {
                            throw!(ALError::InvalidValue);
                        }
                    )*
                )*

                Ok(())
            }

            $(
                $(#[$field_attr])*
                ///
                /// Setter function that checks bounds for the given field if any exist
                pub fn $field(&mut self, $field: $t) -> ALResult<()> {
                    $(
                        if $field < $min || $max < $field {
                            throw!(ALError::InvalidValue);
                        }
                    )*

                    $(
                        if !$check(&$field) {
                            throw!(ALError::InvalidValue);
                        }
                    )*

                    self.$field = $field;

                    Ok(())
                }
            )*
        }

        impl Default for $name {
            fn default() -> $name {
                $name {
                    $(
                        $field: $default,
                    )*
                }
            }
        }
    }
}

efx_struct! {
    /// Properties used for the EAX Reverb effect
    pub struct ALEfxEAXReverbProperties {
        /// [0.0, 1.0]
        pub density: f32                = { 1.0 }    in [0.0, 1.0],
        /// [0.0, 1.0]
        pub diffusion: f32              = { 1.0 }    in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gain: f32                   = { 0.32 }   in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainhf: f32                 = { 0.89 }   in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainlf: f32                 = { 0.0 }    in [0.0, 1.0],
        /// Seconds [0.1, 20.0]
        pub decay_time: f32             = { 1.49 }   in [0.1, 20.0],
        /// [0.1, 2.0]
        pub decay_hfratio: f32          = { 0.83 }   in [0.1, 2.0],
        /// [0.1, 2.0]
        pub decay_lfratio: f32          = { 1.0 }    in [0.1, 2.0],
        /// [0.0, 3.16]
        pub reflections_gain: f32       = { 0.05 }   in [0.0, 3.16],
        /// Seconds [0.0, 0.3]
        pub reflections_delay: f32      = { 0.007 }  in [0.0, 0.3],
        /// [[-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]]
        pub reflections_pan: Vector3<f32> = { Vector3 { x: 0.0, y: 0.0, z: 0.0 } } with |pan: &Vector3<f32>| {
            -1.0 <= pan.x && pan.x <= 1.0 &&
            -1.0 <= pan.y && pan.y <= 1.0 &&
            -1.0 <= pan.z && pan.z <= 1.0
        },
        /// [0.0, 10.0]
        pub late_reverb_gain: f32       = { 1.26 }   in [0.0, 10.0],
        /// Seconds [0.0, 0.1]
        pub late_reverb_delay: f32      = { 0.011 }  in [0.0, 0.1],
        /// [[-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]]
        pub late_reverb_pan: Vector3<f32> = { Vector3 { x: 0.0, y: 0.0, z: 0.0 } } with |pan: &Vector3<f32>| {
            -1.0 <= pan.x && pan.x <= 1.0 &&
            -1.0 <= pan.y && pan.y <= 1.0 &&
            -1.0 <= pan.z && pan.z <= 1.0
        },
        /// [0.075, 0.25]
        pub echo_time: f32              = { 0.25 }   in [0.075, 0.25],
        /// [0.0, 1.0]
        pub echo_depth: f32             = { 0.0 }    in [0.0, 1.0],
        /// [0.04, 4.0]
        pub modulation_time: f32        = { 0.25 }   in [0.04, 4.0],
        /// [0.0, 1.0]
        pub modulation_depth: f32       = { 0.0 }    in [0.0, 1.0],
        /// [0.892, 1.0]
        pub air_absorption_gainhf: f32  = { 0.994 }  in [0.892, 1.0],
        /// Hertz [1000.0, 20_000.0]
        pub hfreference: f32            = { 5000.0 } in [1000.0, 20_000.0],
        /// Hertz [20.0, 1000.0]
        pub lfreference: f32            = { 250.0 }  in [20.0, 1000.0],
        /// [0.0, 10.0]
        pub room_rolloff_factor: f32    = { 0.0 }    in [0.0, 10.0],
        pub decayhf_limit: bool         = { true },
    }
}

impl From<EFXEAXREVERBPROPERTIES> for ALEfxEAXReverbProperties {
    fn from(efx: EFXEAXREVERBPROPERTIES) -> ALEfxEAXReverbProperties {
        ALEfxEAXReverbProperties {
            density: efx.flDensity,
            diffusion: efx.flDiffusion,
            gain: efx.flGain,
            gainhf: efx.flGainHF,
            gainlf: efx.flGainLF,
            decay_time: efx.flDecayTime,
            decay_hfratio: efx.flDecayHFRatio,
            decay_lfratio: efx.flDecayLFRatio,
            reflections_gain: efx.flReflectionsGain,
            reflections_delay: efx.flReflectionsDelay,
            reflections_pan: (&efx.flReflectionsPan).into(),
            late_reverb_gain: efx.flLateReverbGain,
            late_reverb_delay: efx.flLateReverbDelay,
            late_reverb_pan: (&efx.flLateReverbPan).into(),
            echo_time: efx.flEchoTime,
            echo_depth: efx.flEchoDepth,
            modulation_time: efx.flModulationTime,
            modulation_depth: efx.flModulationDepth,
            air_absorption_gainhf: efx.flAirAbsorptionGainHF,
            hfreference: efx.flHFReference,
            lfreference: efx.flLFReference,
            room_rolloff_factor: efx.flRoomRolloffFactor,
            decayhf_limit: efx.iDecayHFLimit as ALboolean == AL_TRUE,
        }
    }
}

impl From<ALEfxEAXReverbProperties> for EFXEAXREVERBPROPERTIES {
    fn from(efx: ALEfxEAXReverbProperties) -> EFXEAXREVERBPROPERTIES {
        EFXEAXREVERBPROPERTIES {
            flDensity: efx.density,
            flDiffusion: efx.diffusion,
            flGain: efx.gain,
            flGainHF: efx.gainhf,
            flGainLF: efx.gainlf,
            flDecayTime: efx.decay_time,
            flDecayHFRatio: efx.decay_hfratio,
            flDecayLFRatio: efx.decay_lfratio,
            flReflectionsGain: efx.reflections_gain,
            flReflectionsDelay: efx.reflections_delay,
            flReflectionsPan: efx.reflections_pan.as_ref().clone(),
            flLateReverbGain: efx.late_reverb_gain,
            flLateReverbDelay: efx.late_reverb_delay,
            flLateReverbPan: efx.late_reverb_pan.as_ref().clone(),
            flEchoTime: efx.echo_time,
            flEchoDepth: efx.echo_depth,
            flModulationTime: efx.modulation_time,
            flModulationDepth: efx.modulation_depth,
            flAirAbsorptionGainHF: efx.air_absorption_gainhf,
            flHFReference: efx.hfreference,
            flLFReference: efx.lfreference,
            flRoomRolloffFactor: efx.room_rolloff_factor,
            iDecayHFLimit: if efx.decayhf_limit { AL_TRUE } else { AL_FALSE } as ::std::os::raw::c_int,
        }
    }
}

efx_struct! {
    /// Properties used for the standard Reverb effect
    pub struct ALEfxReverbProperties {
        /// [0.0, 1.0]
        pub density: f32                = { 1.0 }   in [0.0, 1.0],
        /// [0.0, 1.0]
        pub diffusion: f32              = { 1.0 }   in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gain: f32                   = { 0.32 }  in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainhf: f32                 = { 0.89 }  in [0.0, 1.0],
        /// Seconds [0.1, 20.0]
        pub decay_time: f32             = { 1.49 }  in [0.1, 20.0],
        /// [0.1, 2.0]
        pub decay_hfratio: f32          = { 0.83 }  in [0.1, 2.0],
        /// [0.0, 3.16]
        pub reflections_gain: f32       = { 0.05 }  in [0.0, 3.16],
        /// Seconds [0.0, 0.3]
        pub reflections_delay: f32      = { 0.007 } in [0.0, 0.3],
        /// [0.0, 10.0]
        pub late_reverb_gain: f32       = { 1.26 }  in [0.0, 10.0],
        /// Seconds [0.0, 0.1]
        pub late_reverb_delay: f32      = { 0.011 } in [0.0, 0.1],
        /// [0.892, 1.0]
        pub air_absorption_gainhf: f32  = { 0.994 } in [0.892, 1.0],
        /// [0.0, 10.0]
        pub room_rolloff_factor: f32    = { 0.0 }   in [0.0, 10.0],
        pub decayhf_limit: bool         = { true },
    }
}

impl From<ALEfxReverbProperties> for ALEfxEAXReverbProperties {
    fn from(efx: ALEfxReverbProperties) -> ALEfxEAXReverbProperties {
        ALEfxEAXReverbProperties {
            density: efx.density,
            diffusion: efx.diffusion,
            gain: efx.gain,
            gainhf: efx.gainhf,
            decay_time: efx.decay_time,
            decay_hfratio: efx.decay_hfratio,
            reflections_gain: efx.reflections_gain,
            reflections_delay: efx.reflections_delay,
            late_reverb_gain: efx.late_reverb_gain,
            late_reverb_delay: efx.late_reverb_delay,
            air_absorption_gainhf: efx.air_absorption_gainhf,
            room_rolloff_factor: efx.room_rolloff_factor,
            decayhf_limit: efx.decayhf_limit,
            ..Default::default()
        }
    }
}

impl From<ALEfxEAXReverbProperties> for ALEfxReverbProperties {
    fn from(efx: ALEfxEAXReverbProperties) -> ALEfxReverbProperties {
        ALEfxReverbProperties {
            density: efx.density,
            diffusion: efx.diffusion,
            gain: efx.gain,
            gainhf: efx.gainhf,
            decay_time: efx.decay_time,
            decay_hfratio: efx.decay_hfratio,
            reflections_gain: efx.reflections_gain,
            reflections_delay: efx.reflections_delay,
            late_reverb_gain: efx.late_reverb_gain,
            late_reverb_delay: efx.late_reverb_delay,
            air_absorption_gainhf: efx.air_absorption_gainhf,
            room_rolloff_factor: efx.room_rolloff_factor,
            decayhf_limit: efx.decayhf_limit,
        }
    }
}

impl From<EFXEAXREVERBPROPERTIES> for ALEfxReverbProperties {
    #[inline(always)]
    fn from(efx: EFXEAXREVERBPROPERTIES) -> ALEfxReverbProperties {
        ALEfxEAXReverbProperties::from(efx).into()
    }
}

impl From<ALEfxReverbProperties> for EFXEAXREVERBPROPERTIES {
    #[inline(always)]
    fn from(efx: ALEfxReverbProperties) -> EFXEAXREVERBPROPERTIES {
        ALEfxEAXReverbProperties::from(efx).into()
    }
}

/// Types of waveforms that can be used in the Chorus effect
#[repr(u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEfxChorusWaveform {
    #[serde(rename = "sinusoid")]
    Sinusoid = 0,
    #[serde(rename = "triangle")]
    Triangle = 1,
}

efx_struct! {
    /// Properties used for the Chorus effect
    pub struct ALEfxChorusProperties {
        pub waveform: ALEfxChorusWaveform = { ALEfxChorusWaveform::Triangle },
        /// Degrees [-180, 180]
        pub phase: i16      = { 90 }    in [-180, 180],
        /// Hertz [0.0, 10.0]
        pub rate: f32       = { 1.1 }   in [0.0, 10.0],
        /// [-1.0, 1.0]
        pub depth: f32      = { 0.1 }   in [0.0, 1.0],
        /// Seconds [0.0, 0.016]
        pub feedback: f32   = { 0.25 }  in [-1.0, 1.0],
        /// Seconds [0.0, 0.016]
        pub delay: f32      = { 0.016 } in [0.0, 0.016],
    }
}

efx_struct! {
    /// Properties used for the Distortion effect
    pub struct ALEfxDistortionProperties {
        /// [0.0, 1.0]
        pub edge: f32           = { 0.2 }    in [0.0, 1.0],
        /// [0.01, 1.0]
        pub gain: f32           = { 0.05 }   in [0.01, 1.0],
        /// Hertz [80.0, 24000.0]
        pub lowpass_cutoff: f32 = { 8000.0 } in [80.0, 24000.0],
        /// Hertz [80.0, 24000.0]
        pub eqcenter: f32       = { 3600.0 } in [80.0, 24000.0],
        /// Hertz [80.0, 24000.0]
        pub eqbandwidth: f32    = { 3600.0 } in [80.0, 24000.0],
    }
}

efx_struct! {
    /// Properties used for the Echo effect
    pub struct ALEfxEchoProperties {
        /// Seconds [0.0, 0.207]
        pub delay: f32    = { 0.1 }  in [0.0, 0.207],
        /// Seconds [0.0, 0.404]
        pub lrdelay: f32  = { 0.1 }  in [0.0, 0.404],
        /// [0.0, 0.99]
        pub damping: f32  = { 0.5 }  in [0.0, 0.99],
        /// [0.0, 1.0]
        pub feedback: f32 = { 0.5 }  in [0.0, 1.0],
        /// [-1.0, 1.0]
        pub spread: f32   = { -1.0 } in [-1.0, 1.0],
    }
}

/// Types of waveforms that can be used in the Flanger effect
#[repr(u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEfxFlangerWaveform {
    #[serde(rename = "sinusoid")]
    Sinusoid = 0,
    #[serde(rename = "triangle")]
    Triangle = 1,
}

efx_struct! {
    /// Properties used for the Flanger effect
    pub struct ALEfxFlangerProperties {
        pub waveform: ALEfxFlangerWaveform = { ALEfxFlangerWaveform::Triangle },
        /// Degrees [-180, 180]
        pub phase: i16    = { 0 }     in [-180, 180],
        /// Hertz [0.0, 10.0]
        pub rate: f32     = { 0.27 }  in [0.0, 10.0],
        /// [0.0, 1.0]
        pub depth: f32    = { 1.0 }   in [0.0, 1.0],
        /// [-1.0, 1.0]
        pub feedback: f32 = { -0.5 }  in [-1.0, 1.0],
        /// Seconds [0.0, 0.004]
        pub delay: f32    = { 0.002 } in [0.0, 0.004],
    }
}

/// Directions that can be used in the Frequency Shifter effect
#[repr(u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEfxFrequencyShifterDirection {
    #[serde(rename = "down")]
    Down = 0,
    #[serde(rename = "up")]
    Up = 1,
    #[serde(rename = "off")]
    Off = 2,
}

efx_struct! {
    /// Properties used for the Frequency Shifter effect
    pub struct ALEfxFrequencyShifterProperties {
        /// Hertz [0.0, 24000.0]
        pub frequency: f32 = { 0.0 } in [0.0, 24000.0],
        pub left_direction: ALEfxFrequencyShifterDirection = { ALEfxFrequencyShifterDirection::Down },
        pub right_direction: ALEfxFrequencyShifterDirection = { ALEfxFrequencyShifterDirection::Down },
    }
}

/// Phonemes that can be used in the Vocal Morpher effect
#[repr(u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEfxVocalMorpherPhoneme {
    A = 0,
    E = 1,
    I = 2,
    O = 3,
    U = 4,
    AA = 5,
    AE = 6,
    AH = 7,
    AO = 8,
    EH = 9,
    ER = 10,
    IH = 11,
    IY = 12,
    UH = 13,
    UW = 14,
    B = 15,
    D = 16,
    F = 17,
    G = 18,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    P = 24,
    R = 25,
    S = 26,
    T = 27,
    V = 28,
    Z = 29,
}

/// Types of waveforms that can be used in the Vocal Morpher effect
#[repr(u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEfxVocalMorpherWaveform {
    #[serde(rename = "sinusoid")]
    Sinusoid = 0,
    #[serde(rename = "triangle")]
    Triangle = 1,
    #[serde(rename = "saw")]
    Saw = 2,
}

efx_struct! {
    /// Properties used for the Vocal Morpher effect
    pub struct ALEfxVocalMorpherProperties {
        pub phoneme_a: ALEfxVocalMorpherPhoneme = { ALEfxVocalMorpherPhoneme::A },
        pub phoneme_b: ALEfxVocalMorpherPhoneme = { ALEfxVocalMorpherPhoneme::A },
        /// Semitones [-24, 24]
        pub coarse_tuning_a: i8 = { 0 }    in [-24, 24],
        /// Semitones [-24, 24]
        pub coarse_tuning_b: i8 = { 0 }    in [-24, 24],
        pub waveform: ALEfxVocalMorpherWaveform = { ALEfxVocalMorpherWaveform::Sinusoid },
        /// Hertz [0.0, 10.0]
        pub rate: f32           = { 1.41 } in [0.0, 10.0],
    }
}

efx_struct! {
    /// Properties used for the Pitch Shifter effect
    pub struct ALEfxPitchShifterProperties {
        /// Semitones [-12, 12]
        pub coarse_tune: i8 = { 0 } in [-12, 12],
        /// Cents [-50, 50]
        pub fine_tune: i8   = { 0 } in [-50, 50],
    }
}

/// Types of waveforms that can be used in the Ring Modulator effect
#[repr(u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ALEfxRingModulatorWaveform {
    #[serde(rename = "sinusoid")]
    Sinusoid = 0,
    #[serde(rename = "square")]
    Square = 1,
    #[serde(rename = "saw")]
    Saw = 2,
}

efx_struct! {
    /// Properties used for the Ring Modulator effect
    pub struct ALEfxRingModulatorProperties {
        /// Hertz [0.0, 8_000.0]
        pub frequency: f32       = { 440.0 } in [0.0, 8_000.0],
        /// Hertz [0.0, 24_000.0]
        pub highpass_cutoff: f32 = { 800.0 } in [0.0, 24_000.0],
        pub waveform: ALEfxRingModulatorWaveform = { ALEfxRingModulatorWaveform::Sinusoid },
    }
}

efx_struct! {
    /// Properties used for the Autowah effect
    pub struct ALEfxAutowahProperties {
        /// Seconds [0.0001, 1.0]
        pub attack_time: f32  = { 0.06 }   in [0.0001, 1.0],
        /// Seconds [0.0001, 1.0]
        pub release_time: f32 = { 0.06 }   in [0.0001, 1.0],
        /// [2.0, 1000.0]
        pub resonance: f32    = { 1000.0 } in [2.0, 1000.0],
        /// [0.00003, 31621.0]
        pub peak_gain: f32    = { 11.22 }  in [0.00003, 31621.0],
    }
}

efx_struct! {
    /// Properties used for the Compressor effect
    pub struct ALEfxCompressorProperties {
        pub enabled: bool = { true },
    }
}

efx_struct! {
    /// Properties used for the Equalizer effect
    pub struct ALEfxEqualizerProperties {
        /// [0.126, 7.943]
        pub low_gain: f32       = { 1.0 }       in [0.126, 7.943],
        /// Hertz [50.0, 800.0]
        pub low_cutoff: f32     = { 200.0 }     in [50.0, 800.0],
        /// [0.126, 7.943]
        pub mid1_gain: f32      = { 1.0 }       in [0.126, 7.943],
        /// Hertz [200.0, 3000.0]
        pub mid1_center: f32    = { 500.0 }     in [200.0, 3000.0],
        /// [0.126, 7.943]
        pub mid2_gain: f32      = { 1.0 }       in [0.126, 7.943],
        /// Hertz [1000.0, 8000.0]
        pub mid2_center: f32    = { 3000.0 }    in [1000.0, 8000.0],
        /// [0.126, 7.943]
        pub high_gain: f32      = { 1.0 }       in [0.126, 7.943],
        /// Hertz [4000.0, 16_000.0]
        pub high_cutoff: f32    = { 6000.0 }    in [4000.0, 16_000.0],
    }
}

efx_struct! {
    /// Properties used for the Lowpass filter effect
    pub struct ALEfxLowpassFilterProperties {
        /// [0.0, 1.0]
        pub gain: f32   = { 1.0 } in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainhf: f32 = { 1.0 } in [0.0, 1.0],
    }
}

efx_struct! {
    /// Properties used for the Highpass filter effect
    pub struct ALEfxHighpassFilterProperties {
        /// [0.0, 1.0]
        pub gain: f32   = { 1.0 } in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainlf: f32 = { 1.0 } in [0.0, 1.0],
    }
}

efx_struct! {
    /// Properties used for the Bandpass filter effect
    pub struct ALEfxBandpassFilterProperties {
        /// [0.0, 1.0]
        pub gain: f32   = { 1.0 } in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainlf: f32 = { 1.0 } in [0.0, 1.0],
        /// [0.0, 1.0]
        pub gainhf: f32 = { 1.0 } in [0.0, 1.0],
    }
}
