use als::all::*;

use nalgebra::*;

use std::sync::Arc;
use std::ops::{Deref, DerefMut};

use super::al_error::*;
use super::al_source::*;
use super::al_listener::*;

use super::ALObject;

/// `ALSource3D` is an extension of the normal `ALSource` to provide 3D audio manipulations.
pub struct ALSource3D(Arc<ALSource>);

macro_rules! impl_property {
    ($get_name:ident, $set_name:ident, $name:ident, $t:ident, $alt:ty, $al_enum:ident) => {
        pub fn $get_name(&self) -> ALResult<$t<f32>> {
            try!(self.check());

            let mut $name = $t::new(0.0, 0.0, 0.0);

            unsafe { alGetSourcefv(self.raw(), $al_enum, &mut $name as *mut _ as *mut $alt); }

            check_al_errors!();

            Ok($name)
        }

        pub fn $set_name(&self, $name: $t<f32>) -> ALResult<()> {
            try!(self.check());

            unsafe { alSourcefv(self.raw(), $al_enum, &$name as *const _ as *const $alt); }

            check_al_errors!();

            Ok(())
        }
    }
}

impl ALSource3D {
    #[inline]
    pub fn new() -> ALResult<Arc<ALSource3D>> {
        Ok(ALSource3D::from_source(ALSource::new()?)?)
    }

    /// Convert a normal `ALSource` into an `ALSource3D`,
    /// simultaneously enabling 3D relative positioning of the source.
    pub fn from_source(source: Arc<ALSource>) -> ALResult<Arc<ALSource3D>> {
        let source = ALSource3D(source);

        unsafe { alSourcei(source.raw(), AL_SOURCE_RELATIVE, AL_TRUE as ALint); }

        check_al_errors!();

        Ok(Arc::new(source))
    }

    /// Convert an `ALSource3D` back into an `ALSource`,
    /// simultaneously disabling 3D relative positioning of the source.
    pub fn into_source(self) -> ALResult<Arc<ALSource>> {
        unsafe { alSourcei(self.raw(), AL_SOURCE_RELATIVE, AL_FALSE as ALint); }

        check_al_errors!();

        Ok(self.0)
    }

    impl_property!(get_position, set_position, position, Point3, ALfloat, AL_POSITION);
    impl_property!(get_velocity, set_velocity, velocity, Vector3, ALfloat, AL_VELOCITY);
    impl_property!(get_direction, set_direction, direction, Vector3, ALfloat, AL_DIRECTION);

    /// Set the distance model of this particular source
    ///
    /// **NOTE**: `alEnable(AL_SOURCE_DISTANCE_MODEL)` must be called before this.
    pub fn set_distance_model(&self, model: Option<ALDistanceModel>) -> ALResult<()> {
        try!(self.check());

        unsafe { alSourcei(self.raw(), AL_DISTANCE_MODEL, model.map_or(AL_NONE, |m| m.to_alenum())); }

        check_al_errors!();

        Ok(())
    }
}

impl Deref for ALSource3D {
    type Target = Arc<ALSource>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ALSource3D {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}