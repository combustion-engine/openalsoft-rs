use als::all::*;

use std::mem;
use std::ptr;
use std::sync::Arc;
use std::ops::Deref;
use std::os::raw::c_void;

use super::al_error::*;
use super::al_buffer::*;

use super::ALObject;

pub struct ALSource(ALuint, Vec<Arc<ALBuffer>>);

impl_simple_alobject!(ALSource, alIsSource, "ALSource");

pub enum ALSourceState {
    Initial,
    Paused,
    Playing,
    Stopped,
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
    ($get_name:ident, $set_name:ident, $name:ident, $t:ty, $alt:ty, $al_get_func:ident, $al_set_func:ident, $al_enum:ident) => {
        pub fn $get_name(&self) -> ALResult<$t> {
            try!(self.check());

            let mut $name: $alt = 0.0;

            unsafe { $al_get_func(self.0, $al_enum, &mut $name); }

            check_al_errors!();

            Ok($name as f32)
        }

        pub fn $set_name(&mut self, $name: $t) -> ALResult<()> {
            try!(self.check());

            unsafe { $al_set_func(self.0, $al_enum, $name); }

            check_al_errors!();

            Ok(())
        }
    }
}

impl ALSource {
    pub fn new() -> ALResult<ALSource> {
        let mut source: ALuint = 0;

        unsafe { alGenSources(1, &mut source as *mut _); }

        check_al_errors!();

        Ok(ALSource(source, Vec::new()))
    }

    pub fn set_buffer(&mut self, buffer: Arc<ALBuffer>) -> ALResult<()> {
        try!(self.check());

        unsafe { alSourcei(self.0, AL_BUFFER, buffer.raw() as ALint); }

        check_al_errors!();

        self.1.push(buffer);

        Ok(())
    }

    pub fn queue_buffers<I: Iterator<Item = Arc<ALBuffer>>>(&mut self, buffers: I) -> ALResult<()> {
        try!(self.check());

        for buffer in buffers {
            unsafe { alSourceQueueBuffers(self.0, 1, &buffer.raw() as *const _); }

            check_al_errors!();

            self.1.push(buffer);
        }

        Ok(())
    }

    pub fn unqueue_all_buffers(&mut self) -> ALResult<Vec<Arc<ALBuffer>>> {
        try!(self.check());

        for buffer in &self.1 {
            unsafe { alSourceUnqueueBuffers(self.0, 1, &mut buffer.raw() as *mut _); }

            check_al_errors!();
        }

        let mut empty = Vec::new();

        mem::swap(&mut empty, &mut self.1);

        Ok(empty)
    }

    pub fn set_looping(&mut self, looping: bool) -> ALResult<()> {
        try!(self.check());

        unsafe { alSourcei(self.0, AL_LOOPING, if looping { AL_TRUE } else { AL_FALSE } as ALint); }

        check_al_errors!();

        Ok(())
    }

    impl_simple_func!(play, alSourcePlay);
    impl_simple_func!(pause, alSourcePause);
    impl_simple_func!(stop, alSourceStop);
    impl_simple_func!(rewind, alSourceRewind);

    impl_simple_property!(get_gain, set_gain, gain, f32, ALfloat, alGetSourcef, alSourcef, AL_GAIN);
    impl_simple_property!(get_pitch, set_pitch, pitch, f32, ALfloat, alGetSourcef, alSourcef, AL_PITCH);
}

impl Drop for ALSource {
    fn drop(&mut self) {
        unsafe { alDeleteSources(1, &self.0) }

        ALError::check().unwrap();
    }
}