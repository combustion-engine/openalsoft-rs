pub use als::types;

#[macro_use]
pub mod al_error;

pub use self::al_error::{ALError, ALResult};

pub trait ALObject {
    fn raw(&self) -> types::ALuint;
    fn into_raw(self) -> types::ALuint;
    fn is_valid(&self) -> bool;

    #[inline(always)]
    fn check(&self) -> ALResult<()> {
        if self.is_valid() { Ok(()) } else {
            println!("Invalid ALObject");
            Err(ALError::InvalidValue)
        }
    }
}

#[macro_use]
pub mod macros;

pub mod al_device;
pub mod al_context;
pub mod al_buffer;
pub mod al_source;
pub mod al_source_3d;
pub mod al_listener;
pub mod al_state;
pub mod al_format;
pub mod al_distance_model;

pub use self::al_device::{ALDevice, ALDeviceArc, NULL_DEVICE};
pub use self::al_context::{ALContext, ALContextArc};
pub use self::al_buffer::ALBuffer;
pub use self::al_source::{ALSource, ALSourceKind, ALSourceState};
pub use self::al_source_3d::ALSource3D;
pub use self::al_listener::{ALListener, ALListenerArc};
pub use self::al_state::ALState;
pub use self::al_format::{ALFormat, ALSampleRate, ALBitDepth, ALSampleType, ALChannels};
pub use self::al_distance_model::ALDistanceModel;