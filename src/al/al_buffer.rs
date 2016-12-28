use als::all::*;

use std::mem;
use std::sync::Arc;
use std::os::raw::c_void;
use std::cell::Cell;

use super::al_error::*;
use super::al_format::*;

use super::ALObject;

pub struct ALBuffer(ALuint, Cell<ALFormat>, Cell<usize>);

impl_simple_alobject!(ALBuffer, alIsBuffer, "ALBuffer");

impl ALBuffer {
    pub fn new() -> ALResult<Arc<ALBuffer>> {
        let mut buffer: ALuint = 0;

        unsafe { alGenBuffers(1, &mut buffer as *mut _); }

        check_al_errors!();

        Ok(Arc::new(ALBuffer(buffer, Cell::new(ALFormat::_Uninitialized), Cell::new(0))))
    }

    /// Returns the last number of bytes buffered
    #[inline(always)]
    pub fn num_bytes(&self) -> usize { self.2.get() }

    /// Returns the last number of elements `T` buffered
    ///
    /// It's up to you to keep track of type `T`, as `ALBuffer` really only stores the number of bytes, not elements, buffered.
    #[inline(always)]
    pub fn num_elements<T>(&self) -> usize { self.2.get() / mem::size_of::<T>() }


    /// Buffer a `Vec<T>` of elements `T` to the `ALBuffer`
    #[inline]
    pub fn buffer_elements<T>(&self, data: &Vec<T>, format: ALFormat) -> ALResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), format) }
    }

    /// Buffer a slice of `T` to the `ALBuffer`
    #[inline]
    pub fn buffer_slice<T>(&self, data: &[T], format: ALFormat) -> ALResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), format) }
    }

    /// Buffer raw data to the `ALBuffer`
    pub unsafe fn buffer_raw(&self, data: *const c_void, size: usize, format: ALFormat) -> ALResult<()> {
        if data.is_null() || size == 0 {
            Err(ALError::InvalidValue)
        } else {
            try!(self.check());

            let (al_format, srate) = match format {
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

            alBufferData(self.0, al_format, data, size as ALsizei, srate as ALsizei);

            check_al_errors!();

            self.1.set(format);
            self.2.set(size);

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