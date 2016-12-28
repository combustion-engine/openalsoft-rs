pub use als::types;

#[macro_use]
pub mod al_error;

pub use self::al_error::*;

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

pub use self::al_device::*;
pub use self::al_context::*;
pub use self::al_buffer::*;
pub use self::al_source::*;
pub use self::al_source_3d::*;
pub use self::al_listener::*;
pub use self::al_state::*;
pub use self::al_format::*;