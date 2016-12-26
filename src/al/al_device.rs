use als::all::*;

use std::ptr;
use std::sync::Arc;
use std::borrow::Cow;
use std::ffi::{CString, CStr};

use super::al_error::*;
use super::al_context::*;

pub struct ALDevice {
    raw: *mut ALCdevice,
}

impl ALDevice {
    #[inline(always)]
    pub unsafe fn raw(&self) -> *mut ALCdevice { self.raw }

    pub unsafe fn open() -> ALResult<Arc<ALDevice>> {
        let device = alcOpenDevice(ptr::null());

        if device.is_null() {
            check_alc_errors!();

            panic!("Could not create OpenAL device");
        }

        Ok(Arc::new(ALDevice { raw: device }))
    }

    pub fn extension_present(&self, extension: &str) -> ALResult<bool> {
        let c_ext = CString::new(extension)?;

        let res = unsafe {
            alcIsExtensionPresent(self.raw, c_ext.as_ptr() as *const _)
        };

        check_alc_errors!();

        Ok(res == ALC_TRUE)
    }

    pub fn get_string(&self, param: ALenum) -> ALResult<Cow<str>> {
        let c_str = unsafe {
            alcGetString(self.raw, param)
        };

        check_alc_errors!();

        Ok(unsafe {
            CStr::from_ptr(c_str).to_string_lossy()
        })
    }
}

impl Drop for ALDevice {
    fn drop(&mut self) {
        unsafe { alcCloseDevice(self.raw); }

        ALError::check_alc().unwrap();
    }
}
