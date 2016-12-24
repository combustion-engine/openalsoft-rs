use als::all::*;

use std::mem;
use std::ptr;
use std::os::raw::c_void;

use super::al_error::*;
use super::al_device::*;
use super::ALObject;

pub struct ALBuffer(ALuint, ALFormat, usize);

impl_simple_alobject!(ALBuffer, alIsBuffer, "ALBuffer");

pub type ALSampleRate = u32;

#[derive(Debug, Copy, Clone)]
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

impl ALBuffer {
    pub fn new() -> ALResult<ALBuffer> {
        let mut buffer: ALuint = 0;

        unsafe { alGenBuffers(1, &mut buffer as *mut _); }

        check_al_errors!();

        Ok(ALBuffer(buffer, ALFormat::_Uninitialized, 0))
    }

    /// Returns the last number of bytes buffered
    #[inline(always)]
    pub fn num_bytes(&self) -> usize { self.2 }

    /// Returns the last number of elements `T` buffered
    ///
    /// It's up to you to keep track of type `T`, as `ALBuffer` really only stores the number of bytes, not elements, buffered.
    #[inline(always)]
    pub fn num_elements<T>(&self) -> usize { self.2 / mem::size_of::<T>() }


    /// Buffer a `Vec<T>` of elements `T` to the `ALBuffer`
    #[inline]
    pub fn buffer_elements<T>(&mut self, data: &Vec<T>, format: ALFormat) -> ALResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), format) }
    }

    /// Buffer a slice of `T` to the `ALBuffer`
    #[inline]
    pub fn buffer_slice<T>(&mut self, data: &[T], format: ALFormat) -> ALResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), format) }
    }

    /// Buffer raw data to the `ALBuffer`
    pub unsafe fn buffer_raw(&mut self, data: *const c_void, size: usize, format: ALFormat) -> ALResult<()> {
        if data.is_null() || size == 0 {
            Err(ALError::InvalidValue)
        } else {
            try!(self.check());

            let (format, srate) = match format {
                ALFormat::Mono8(srate) => (AL_FORMAT_MONO8, srate),
                ALFormat::Mono16(srate) => (AL_FORMAT_MONO16, srate),
                ALFormat::Stereo8(srate) => (AL_FORMAT_STEREO8, srate),
                ALFormat::Stereo16(srate) => (AL_FORMAT_STEREO16, srate),
                ALFormat::MonoFloat32(srate) => (AL_FORMAT_MONO_FLOAT32, srate),
                ALFormat::StereoFloat32(srate) => (AL_FORMAT_STEREO_FLOAT32, srate),

                ALFormat::Unsupported | ALFormat::_Uninitialized => {
                    return Err(ALError::Unsupported);
                }
            };

            alBufferData(self.0, format, data, size as ALsizei, srate as ALsizei);

            check_al_errors!();

            self.2 = size;

            Ok(())
        }
    }

    pub fn format(&self) -> ALResult<ALFormat> {
        try!(self.check());

        let mut freq = 0;
        let mut bits = 0;
        let mut channels = 0;

        unsafe {
            alGetBufferi(self.0, AL_FREQUENCY, &mut freq);
            alGetBufferi(self.0, AL_BITS, &mut bits);
            alGetBufferi(self.0, AL_CHANNELS, &mut channels);
        }

        check_al_errors!();

        Ok(ALFormat::from_parts(bits as usize, channels as usize, freq as ALSampleRate))
    }
}

impl Drop for ALBuffer {
    fn drop(&mut self) {
        unsafe { alDeleteBuffers(1, &self.0); }

        ALError::check().unwrap();
    }
}