use als::all::*;

use nalgebra::*;

use std::mem;
use std::ptr;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;

use super::al_error::*;
use super::al_source::*;

use super::ALObject;

pub struct ALSource3D(ALSource);

impl ALSource3D {
    #[inline]
    pub fn new() -> ALResult<ALSource3D> {
        Ok(ALSource3D::from_source(ALSource::new()?)?)
    }

    pub fn from_source(source: ALSource) -> ALResult<ALSource3D> {
        let source = ALSource3D(source);

        unsafe { alSourcei(source.raw(), AL_SOURCE_RELATIVE, AL_TRUE as ALint); }

        check_al_errors!();

        Ok(source)
    }

    pub fn into_source(self) -> ALResult<ALSource> {
        unsafe { alSourcei(self.raw(), AL_SOURCE_RELATIVE, AL_FALSE as ALint); }

        check_al_errors!();

        Ok(self.0)
    }

    pub fn set_position(&mut self, pos: Point3<f32>) -> ALResult<()> {
        try!(self.check());

        unsafe { alSource3f(self.raw(), AL_POSITION, pos.x as ALfloat, pos.y as ALfloat, pos.z as ALfloat); }

        check_al_errors!();

        Ok(())
    }

    pub fn set_velocity(&mut self, velocity: Vector3<f32>) -> ALResult<()> {
        try!(self.check());

        unsafe { alSource3f(self.raw(), AL_VELOCITY, velocity.x as ALfloat, velocity.y as ALfloat, velocity.z as ALfloat) }

        check_al_errors!();

        Ok(())
    }

    pub fn set_direction(&mut self, direction: Vector3<f32>) -> ALResult<()> {
        try!(self.check());

        unsafe { alSource3f(self.raw(), AL_DIRECTION, direction.x as ALfloat, direction.y as ALfloat, direction.z as ALfloat) }

        check_al_errors!();

        Ok(())
    }
}

impl Deref for ALSource3D {
    type Target = ALSource;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ALSource3D {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}