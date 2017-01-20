use als::all::*;

use std::error::Error;
use std::io;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ffi::{NulError};
use std::string::FromUtf8Error;
use std::sync::atomic::{Ordering, AtomicBool, ATOMIC_BOOL_INIT};

use trace_error::TraceResult;

pub type ALResult<T> = TraceResult<T, ALError>;

#[derive(Debug)]
pub enum ALError {
    //OpenAL Errors
    InvalidName,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidDevice,
    InvalidContext,
    OutOfMemory,
    //Rust errors
    UnknownError(ALenum),
    UnknownContextError(ALCenum),
    NulError(NulError),
    Io(io::Error),
    FromUtf8Error(FromUtf8Error),
    //Other errors
    Unsupported,
}

static mut CHECK_DISABLED: AtomicBool = ATOMIC_BOOL_INIT;


#[macro_export]
macro_rules! check_al_errors {
    () => {try_rethrow!(ALError::check())};

    ($ret:expr) => {
        try_rethrow!(ALError::check());

        $ret
    };
}

#[macro_export]
macro_rules! check_alc_errors {
    () => {try_rethrow!(ALError::check_alc())};

    ($ret:expr) => {
        try_rethrow!(ALError::check_alc());

        $ret
    };
}

impl ALError {
    /// Check if there are any errors in the OpenAL error queue
    ///
    /// If check was disabled, this functions returns `Ok(())` immediately
    pub fn check() -> ALResult<()> {
        if unsafe { !CHECK_DISABLED.load(Ordering::SeqCst) } {
            let err = unsafe { alGetError() };

            if err != AL_NO_ERROR {
                throw!(match err {
                    AL_INVALID_NAME => ALError::InvalidName,
                    AL_INVALID_ENUM => ALError::InvalidEnum,
                    AL_INVALID_VALUE => ALError::InvalidValue,
                    AL_INVALID_OPERATION => ALError::InvalidOperation,
                    AL_OUT_OF_MEMORY => ALError::OutOfMemory,
                    _ => ALError::UnknownError(err),
                });
            }

            try_rethrow!(ALError::check_alc());
        }

        Ok(())
    }

    pub fn check_alc() -> ALResult<()> {
        if unsafe { !CHECK_DISABLED.load(Ordering::SeqCst) } {
            let ctx = unsafe { alcGetCurrentContext() };

            if !ctx.is_null() {
                let device = unsafe { alcGetContextsDevice(ctx) };

                if !device.is_null() {
                    let err = unsafe { alcGetError(device) };

                    if err != ALC_NO_ERROR {
                        throw!(match err {
                            ALC_INVALID_DEVICE => ALError::InvalidDevice,
                            ALC_INVALID_CONTEXT => ALError::InvalidContext,
                            ALC_INVALID_ENUM => ALError::InvalidEnum,
                            ALC_INVALID_VALUE => ALError::InvalidValue,
                            ALC_OUT_OF_MEMORY => ALError::OutOfMemory,
                            _ => ALError::UnknownContextError(err),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Disable the `check` function, causing it to return `Ok(())` instantly every time.
    ///
    /// The only real reason to do this is to improve performance in very hot loops,
    /// just don't forget to re-enable it and check as soon as possible.
    pub unsafe fn disable_check() {
        CHECK_DISABLED.store(true, Ordering::SeqCst);
    }

    /// Enable the `check` function, resuming its normal behavior after it had been disabled
    pub unsafe fn enable_check() {
        CHECK_DISABLED.store(false, Ordering::SeqCst);
    }
}


impl From<NulError> for ALError {
    fn from(err: NulError) -> ALError {
        ALError::NulError(err)
    }
}

impl From<io::Error> for ALError {
    fn from(err: io::Error) -> ALError {
        ALError::Io(err)
    }
}

impl From<FromUtf8Error> for ALError {
    fn from(err: FromUtf8Error) -> ALError {
        ALError::FromUtf8Error(err)
    }
}

impl From<ALError> for io::Error {
    fn from(err: ALError) -> io::Error {
        match err {
            ALError::Io(io_err) => io_err,
            _ => io::Error::new(io::ErrorKind::Other, err)
        }
    }
}

impl Display for ALError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for ALError {
    fn description(&self) -> &str {
        match *self {
            ALError::NulError(ref err) => err.description(),
            ALError::Io(ref err) => err.description(),
            ALError::FromUtf8Error(ref err) => err.description(),
            ALError::InvalidName => "Invalid Name",
            ALError::InvalidEnum => "Invalid Enum",
            ALError::InvalidValue => "Invalid Value",
            ALError::InvalidOperation => "Invalid Operation",
            ALError::InvalidDevice => "Invalid Device",
            ALError::InvalidContext => "Invalid Context",
            ALError::OutOfMemory => "Out of Memory",
            ALError::UnknownError(_) => "Unknown OpenAL Error",
            ALError::UnknownContextError(_) => "Unknown OpenAL Context Error",
            ALError::Unsupported => "Unsupported Feature",
        }
    }
}