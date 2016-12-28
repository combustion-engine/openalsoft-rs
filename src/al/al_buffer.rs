use als::all::*;

use std::mem;
use std::sync::Arc;
use std::os::raw::c_void;
use std::cell::Cell;

use super::al_error::*;
use super::al_format::*;

use super::ALObject;

#[derive(PartialEq, Eq)]
pub struct ALBuffer(ALuint, Cell<Option<ALFormat>>, Cell<usize>);

impl_simple_alobject!(ALBuffer, alIsBuffer, "ALBuffer");

impl ALBuffer {
    pub fn new() -> ALResult<Arc<ALBuffer>> {
        let mut buffer: ALuint = 0;

        unsafe { alGenBuffers(1, &mut buffer as *mut _); }

        check_al_errors!();

        Ok(Arc::new(ALBuffer(buffer, Cell::new(None), Cell::new(0))))
    }

    pub fn from_elements<T>(data: &Vec<T>, format: ALFormat) -> ALResult<Arc<ALBuffer>> {
        let buffer = ALBuffer::new()?;

        buffer.buffer_elements(data, format)?;

        Ok(buffer)
    }

    pub fn from_slice<T>(data: &[T], format: ALFormat) -> ALResult<Arc<ALBuffer>> {
        let buffer = ALBuffer::new()?;

        buffer.buffer_slice(data, format)?;

        Ok(buffer)
    }

    pub unsafe fn from_raw(data: *const c_void, size: usize, samples: usize, format: ALFormat) -> ALResult<Arc<ALBuffer>> {
        let buffer = ALBuffer::new()?;

        buffer.buffer_raw(data, size, samples, format)?;

        Ok(buffer)
    }

    /// Returns the last number of bytes buffered
    #[inline(always)]
    pub fn num_bytes(&self) -> usize { self.2.get() }

    /// Buffer a `Vec<T>` of elements `T` to the `ALBuffer`
    #[inline]
    pub fn buffer_elements<T>(&self, data: &Vec<T>, format: ALFormat) -> ALResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), data.len(), format) }
    }

    /// Buffer a slice of `T` to the `ALBuffer`
    #[inline]
    pub fn buffer_slice<T>(&self, data: &[T], format: ALFormat) -> ALResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), data.len(), format) }
    }

    /// Buffer raw data to the `ALBuffer`
    pub unsafe fn buffer_raw(&self, data: *const c_void, size: usize, samples: usize, format: ALFormat) -> ALResult<()> {
        if data.is_null() || size == 0 {
            Err(ALError::InvalidValue)
        } else {
            try!(self.check());

            let internal_format = format.internal_format();
            let channels = format.channels();
            let sample_type = format.sample_type();

            alBufferSamplesSOFT(self.0, format.sample_rate as ALuint, internal_format, samples as ALsizei, channels, sample_type, data);

            check_al_errors!();

            self.1.set(Some(format));
            self.2.set(size);

            Ok(())
        }
    }
}

impl Drop for ALBuffer {
    fn drop(&mut self) {
        unsafe { alDeleteBuffers(1, &self.0); }

        ALError::check().unwrap();
    }
}