pub type ALSampleRate = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ALFormat {
    Unsupported,
    Mono8(ALSampleRate),
    Mono16(ALSampleRate),
    Stereo8(ALSampleRate),
    Stereo16(ALSampleRate),
    MonoFloat32(ALSampleRate),
    StereoFloat32(ALSampleRate),
    #[doc(hidden)]
    _Uninitialized,
}

impl ALFormat {
    pub fn from_parts(bits: usize, channels: usize, srate: ALSampleRate) -> ALFormat {
        match channels {
            1 => {
                match bits {
                    8 => ALFormat::Mono8(srate),
                    16 => ALFormat::Mono16(srate),
                    32 => ALFormat::MonoFloat32(srate),
                    _ => ALFormat::Unsupported,
                }
            },
            2 => {
                match bits {
                    8 => ALFormat::Stereo8(srate),
                    16 => ALFormat::Stereo16(srate),
                    32 => ALFormat::StereoFloat32(srate),
                    _ => ALFormat::Unsupported,
                }
            }
            _ => ALFormat::Unsupported,
        }
    }

    pub fn bits(&self) -> Option<usize> {
        match *self {
            ALFormat::Mono8(_) | ALFormat::Stereo8(_) => Some(8),
            ALFormat::Mono16(_) | ALFormat::Stereo16(_) => Some(16),
            ALFormat::MonoFloat32(_) | ALFormat::StereoFloat32(_) => Some(32),
            _ => None,
        }
    }

    pub fn sample_rate(&self) -> Option<ALSampleRate> {
        match *self {
            ALFormat::Mono8(srate) => Some(srate),
            ALFormat::Mono16(srate) => Some(srate),
            ALFormat::Stereo8(srate) => Some(srate),
            ALFormat::Stereo16(srate) => Some(srate),
            ALFormat::MonoFloat32(srate) => Some(srate),
            ALFormat::StereoFloat32(srate) => Some(srate),
            _ => None,
        }
    }

    pub fn channels(&self) -> Option<usize> {
        match *self {
            ALFormat::Mono8(_) | ALFormat::Mono16(_) | ALFormat::MonoFloat32(_) => Some(1),
            ALFormat::Stereo8(_) | ALFormat::Stereo16(_) | ALFormat::StereoFloat32(_) => Some(2),
            _ => None,
        }
    }
}
