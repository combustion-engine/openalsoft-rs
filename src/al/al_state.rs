use als::all::*;

use std::borrow::Cow;
use std::ffi::{CStr, CString};

use super::al_error::*;

// Provides safe access to the global OpenAL state.
pub struct ALState;

impl ALState {
    pub fn enable(param: ALenum) -> ALResult<()> {
        unsafe { alEnable(param); }

        check_al_errors!();

        Ok(())
    }

    pub fn disable(param: ALenum) -> ALResult<()> {
        unsafe { alDisable(param); }

        check_al_errors!();

        Ok(())
    }

    pub fn is_enabled(param: ALenum) -> ALResult<bool> {
        let res = unsafe { alIsEnabled(param) };

        check_al_errors!();

        Ok(res == AL_TRUE)
    }

    pub fn get_string<'a>(param: ALenum) -> ALResult<Cow<'a, str>> {
        let c_str = unsafe { alGetString(param) };

        check_al_errors!();

        Ok(unsafe { CStr::from_ptr(c_str).to_string_lossy() })
    }

    pub fn get_integer(param: ALenum) -> ALResult<ALint> {
        let res = unsafe { alGetInteger(param) };

        check_al_errors!();

        Ok(res)
    }

    pub fn get_enum(name: &str) -> ALResult<ALenum> {
        let c_str = CString::new(name)?;

        let res = unsafe { alGetEnumValue(c_str.as_ptr()) };

        check_al_errors!();

        if res == 0 || res == -1 {
            Err(ALError::InvalidEnum)
        } else {
            Ok(res)
        }
    }
}