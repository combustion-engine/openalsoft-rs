use als::all::*;

use std::ptr;
use std::marker::PhantomData;
use std::borrow::Cow;
use std::ffi::{CString, CStr};

use super::al_error::*;
use super::al_context::*;

pub struct ALDevice<'a> {
    raw: *mut ALCdevice,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ALDevice<'a> {
    #[inline(always)]
    pub unsafe fn raw(&self) -> *mut ALCdevice { self.raw }

    pub unsafe fn open() -> ALResult<ALDevice<'a>> {
        let device = alcOpenDevice(ptr::null());

        if device.is_null() {
            check_alc_errors!();

            panic!("Could not create OpenAL device");
        }

        Ok(ALDevice { raw: device, _marker: PhantomData })
    }

    #[inline]
    pub fn create_context<'b>(&mut self) -> ALResult<ALContext<'b>> where 'a: 'b {
        ALContext::create_from_device(self)
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

impl<'a> Drop for ALDevice<'a> {
    fn drop(&mut self) {
        unsafe { alcCloseDevice(self.raw); }

        ALError::check_alc().unwrap();
    }
}
