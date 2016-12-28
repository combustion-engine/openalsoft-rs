use als::all::*;

use std::mem;
use std::sync::Arc;
use std::cell::RefCell;

use super::al_error::*;
use super::al_buffer::*;
use super::al_listener::*;

use super::ALObject;

pub struct ALSource(ALuint, RefCell<Vec<Arc<ALBuffer>>>, Arc<ALListener>);

impl_simple_alobject!(ALSource, alIsSource, "ALSource");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ALSourceState {
    Initial,
    Paused,
    Playing,
    Stopped,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ALSourceKind {
    Undetermined,
    Static,
    Streaming
}

macro_rules! impl_simple_func {
    ($name:ident, $al_name:ident) => {
        pub fn $name(&self) -> ALResult<()> {
            try!(self.check());

            unsafe { $al_name(self.0); }

            check_al_errors!();

            Ok(())
        }
    }
}

macro_rules! impl_simple_property {
    ($get_name:ident, $set_name:ident, $name:ident, $t:ty, $alt:ty, $al_enum:ident) => {
        pub fn $get_name(&self) -> ALResult<$t> {
            try!(self.check());

            let mut $name: $alt = 0.0;

            unsafe { alGetSourcef(self.0, $al_enum, &mut $name); }

            check_al_errors!();

            Ok($name as f32)
        }

        pub fn $set_name(&self, $name: $t) -> ALResult<()> {
            try!(self.check());

            unsafe { alSourcef(self.0, $al_enum, $name); }

            check_al_errors!();

            Ok(())
        }
    }
}

impl ALSource {
    pub fn new(listener: Arc<ALListener>) -> ALResult<Arc<ALSource>> {
        let mut source: ALuint = 0;

        unsafe { alGenSources(1, &mut source as *mut _); }

        check_al_errors!();

        Ok(Arc::new(ALSource(source, RefCell::new(Vec::new()), listener)))
    }

    pub fn kind(&self) -> ALResult<ALSourceKind> {
        let mut kind = 0;

        unsafe { alGetSourcei(self.0, AL_SOURCE_TYPE, &mut kind); }

        check_al_errors!();

        Ok(match kind {
            AL_UNDETERMINED => ALSourceKind::Undetermined,
            AL_STATIC => ALSourceKind::Static,
            AL_STREAMING => ALSourceKind::Streaming,
            _ => return Err(ALError::InvalidValue)
        })
    }

    /// Add a buffer to the streaming queue
    pub fn queue_buffers<I: Iterator<Item = Arc<ALBuffer>>>(&self, buffer_iter: I) -> ALResult<()> {
        try!(self.check());

        let mut buffers = self.1.borrow_mut();

        for buffer in buffer_iter {
            unsafe { alSourceQueueBuffers(self.0, 1, &buffer.raw() as *const _); }

            check_al_errors!();

            buffers.push(buffer);
        }

        Ok(())
    }

    pub fn unqueue_buffer(&self, buffer: Arc<ALBuffer>) -> ALResult<bool> {
        try!(self.check());

        let mut buffers = self.1.borrow_mut();

        Ok(if let Some(position) = buffers.iter().position(|b| *b == buffer) {
            unsafe { alSourceUnqueueBuffers(self.0, 1, &mut buffer.raw() as *mut _); }

            check_al_errors!();

            buffers.remove(position);

            true
        } else { false })
    }

    /// Remove all queued buffers
    pub fn unqueue_all_buffers(&self) -> ALResult<Vec<Arc<ALBuffer>>> {
        try!(self.check());

        let mut buffers = self.1.borrow_mut();

        for buffer in buffers.iter() {
            unsafe { alSourceUnqueueBuffers(self.0, 1, &mut buffer.raw() as *mut _); }

            check_al_errors!();
        }

        let mut new = Vec::new();

        mem::swap(&mut new, &mut *buffers);

        Ok(new)
    }

    /// Get all buffers that are actively queued.
    pub fn buffers(&self) -> Vec<Arc<ALBuffer>> {
        self.1.borrow().clone()
    }

    /// Returns the number of buffers queued to OpenAL.
    ///
    /// This number might not be equal to `source.buffers().len()`,
    /// as this is the count maintained by OpenAL itself.
    pub fn buffers_queued(&self) -> ALResult<usize> {
        try!(self.check());

        let mut count = 0;

        unsafe { alGetSourcei(self.0, AL_BUFFERS_QUEUED, &mut count); }

        check_al_errors!();

        Ok(count as usize)
    }

    /// Returns the number of buffers that have been played.
    pub fn buffers_processed(&self) -> ALResult<usize> {
        try!(self.check());

        let mut count = 0;

        unsafe { alGetSourcei(self.0, AL_BUFFERS_PROCESSED, &mut count); }

        check_al_errors!();

        Ok(count as usize)
    }

    pub fn set_looping(&self, looping: bool) -> ALResult<()> {
        try!(self.check());

        unsafe { alSourcei(self.0, AL_LOOPING, if looping { AL_TRUE } else { AL_FALSE } as ALint); }

        check_al_errors!();

        Ok(())
    }

    impl_simple_func!(play, alSourcePlay);
    impl_simple_func!(pause, alSourcePause);
    impl_simple_func!(stop, alSourceStop);
    impl_simple_func!(rewind, alSourceRewind);

    /// Get the active source state
    pub fn state(&self) -> ALResult<ALSourceState> {
        try!(self.check());

        let mut state = 0;

        unsafe { alGetSourcei(self.0, AL_SOURCE_STATE, &mut state); }

        Ok(match state {
            AL_INITIAL => ALSourceState::Initial,
            AL_PAUSED => ALSourceState::Paused,
            AL_PLAYING => ALSourceState::Playing,
            AL_STOPPED => ALSourceState::Stopped,
            _ => return Err(ALError::InvalidValue)
        })
    }

    #[inline]
    pub fn is_playing(&self) -> ALResult<bool> {
        Ok(self.state()? == ALSourceState::Playing)
    }

    impl_simple_property!(get_gain, set_gain, gain, f32, ALfloat, AL_GAIN);
    impl_simple_property!(get_pitch, set_pitch, pitch, f32, ALfloat, AL_PITCH);
}

impl Drop for ALSource {
    fn drop(&mut self) {
        unsafe { alDeleteSources(1, &self.0) }

        ALError::check().unwrap();
    }
}