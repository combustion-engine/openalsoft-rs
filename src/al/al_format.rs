use als::all::*;

pub type ALSampleRate = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ALChannels {
    /// Monophonic sample
    Mono,
    /// Stereo sample
    Stereo,
    /// Quadraphonic sample
    Quad,
    /// Rear speaker sample
    Rear,
    /// 5.1 Surround Sound sample
    Point51,
    /// 6.1 Surround Sound sample
    Point61,
    /// 7.1 Surround Sound sample
    Point71,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ALBitDepth {
    /// 8-bit integer format
    ///
    /// Allows 256 discrete values
    Int8,
    /// 16-bit integer format
    ///
    /// Allows 65,536 discrete values
    Int16,
    /// 32-bit floating point format
    ///
    /// Allows any value between -1.0 and 1.0 with a good degree of accuracy
    Float32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ALSampleType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
    Double,
    Byte3,
    UnsignedByte3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ALFormat {
    /// Internal bit-depth
    pub depth: ALBitDepth,
    /// Audio channels
    pub channels: ALChannels,
    /// Sample rate in Hz
    pub sample_rate: ALSampleRate,
    /// Sample type
    pub sample_type: ALSampleType,
}

impl ALFormat {
    pub fn common_stereo32f(sample_rate: ALSampleRate) -> ALFormat {
        ALFormat {
            depth: ALBitDepth::Float32,
            channels: ALChannels::Stereo,
            sample_rate: sample_rate,
            sample_type: ALSampleType::Float,
        }
    }

    pub fn bit_depth_as_bytes(&self) -> usize {
        match self.depth {
            ALBitDepth::Int8 => 1,
            ALBitDepth::Int16 => 2,
            ALBitDepth::Float32 => 4,
        }
    }

    pub fn internal_format(&self) -> ALenum {
        match self.depth {
            ALBitDepth::Int8 => {
                match self.channels {
                    ALChannels::Mono => AL_MONO8_SOFT,
                    ALChannels::Stereo => AL_STEREO8_SOFT,
                    ALChannels::Quad => AL_QUAD8_SOFT,
                    ALChannels::Rear => AL_REAR8_SOFT,
                    ALChannels::Point51 => AL_5POINT1_8_SOFT,
                    ALChannels::Point61 => AL_6POINT1_8_SOFT,
                    ALChannels::Point71 => AL_7POINT1_8_SOFT,
                }
            },
            ALBitDepth::Int16 => {
                match self.channels {
                    ALChannels::Mono => AL_MONO16_SOFT,
                    ALChannels::Stereo => AL_STEREO16_SOFT,
                    ALChannels::Quad => AL_QUAD16_SOFT,
                    ALChannels::Rear => AL_REAR16_SOFT,
                    ALChannels::Point51 => AL_5POINT1_16_SOFT,
                    ALChannels::Point61 => AL_6POINT1_16_SOFT,
                    ALChannels::Point71 => AL_7POINT1_16_SOFT,
                }
            },
            ALBitDepth::Float32 => {
                match self.channels {
                    ALChannels::Mono => AL_MONO32F_SOFT,
                    ALChannels::Stereo => AL_STEREO32F_SOFT,
                    ALChannels::Quad => AL_QUAD32F_SOFT,
                    ALChannels::Rear => AL_REAR32F_SOFT,
                    ALChannels::Point51 => AL_5POINT1_32F_SOFT,
                    ALChannels::Point61 => AL_6POINT1_32F_SOFT,
                    ALChannels::Point71 => AL_7POINT1_32F_SOFT,
                }
            }
        }
    }

    pub fn channels(&self) -> ALenum {
        match self.channels {
            ALChannels::Mono => AL_MONO_SOFT,
            ALChannels::Stereo => AL_STEREO_SOFT,
            ALChannels::Quad => AL_QUAD_SOFT,
            ALChannels::Rear => AL_REAR_SOFT,
            ALChannels::Point51 => AL_5POINT1_SOFT,
            ALChannels::Point61 => AL_6POINT1_SOFT,
            ALChannels::Point71 => AL_7POINT1_SOFT,
        }
    }

    pub fn sample_type(&self) -> ALenum {
        match self.sample_type {
            ALSampleType::Byte => AL_BYTE_SOFT,
            ALSampleType::UnsignedByte => AL_UNSIGNED_BYTE_SOFT,
            ALSampleType::Short => AL_SHORT_SOFT,
            ALSampleType::UnsignedShort => AL_UNSIGNED_SHORT_SOFT,
            ALSampleType::Int => AL_INT_SOFT,
            ALSampleType::UnsignedInt => AL_UNSIGNED_INT_SOFT,
            ALSampleType::Float => AL_FLOAT_SOFT,
            ALSampleType::Double => AL_DOUBLE_SOFT,
            ALSampleType::Byte3 => AL_BYTE3_SOFT,
            ALSampleType::UnsignedByte3 => AL_UNSIGNED_BYTE3_SOFT,
        }
    }
}