pub use als::types;

#[macro_use]
pub mod error;

pub use self::error::*;

pub trait ALObject {
    fn raw(&self) -> types::ALuint;
    fn into_raw(self) -> types::ALuint;
    fn is_valid(&self) -> bool;

    #[inline(always)]
    fn check(&self) -> ALResult<()> {
        if self.is_valid() { Ok(()) } else {
            throw!(ALError::InvalidValue);
        }
    }
}

macro_rules! impl_simple_alobject {
    (simple $name:ident, $is:ident $(, { $extra_cond:expr } )*) => {
        impl $crate::al::ALObject for $name {
            #[inline(always)]
            fn raw(&self) -> ALuint { self.0 }

            #[inline(always)]
            fn into_raw(mut self) -> ALuint {
                ::std::mem::replace(&mut self.0, 0)
            }

            #[inline(always)]
            fn is_valid(&self) -> bool {
                $($extra_cond(self) ||)* ::als::consts::AL_TRUE == unsafe { ::als::ffi::$is(self.0) }
            }
        }
    };

    (struct $name:ident, $is:ident $(, { $extra_cond:expr } )*) => {
        impl $crate::al::ALObject for $name {
            #[inline(always)]
            fn raw(&self) -> ALuint { self.raw }

            #[inline(always)]
            fn into_raw(mut self) -> ALuint {
                ::std::mem::replace(&mut self.raw, 0)
            }

            #[inline(always)]
            fn is_valid(&self) -> bool {
                $($extra_cond(self) ||)* ::als::consts::AL_TRUE == unsafe { ::als::ffi::$is(self.raw) }
            }
        }
    };
}

pub mod device;
pub mod context;
pub mod buffer;
pub mod source;
pub mod source_3d;
pub mod listener;
pub mod state;
pub mod format;
pub mod distance_model;
pub mod effect;
pub mod effects;

pub use self::device::{ALDevice, ALDeviceArc, NULL_DEVICE};
pub use self::context::{ALContext, ALContextArc};
pub use self::buffer::ALBuffer;
pub use self::source::{ALSource, ALSourceKind, ALSourceState};
pub use self::source_3d::ALSource3D;
pub use self::listener::{ALListener, ALListenerArc};
pub use self::state::ALState;
pub use self::format::{ALFormat, ALSampleRate, ALBitDepth, ALSampleType, ALChannels};
pub use self::distance_model::ALDistanceModel;