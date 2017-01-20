use als::all::*;
use als::ext::ALC_ENUMERATE_ALL_EXT_NAME;

use std::ptr;
use std::sync::Arc;
use std::borrow::Cow;
use std::ffi::{CString, CStr};

use super::error::*;
use super::context::*;
use super::listener::*;

lazy_static! {
    /// `NULL_DEVICE` is useful for checking system capabilities before creating a real device instance.
    pub static ref NULL_DEVICE: Arc<ALDevice> = Arc::new(ALDevice { raw: ptr::null_mut() });
}

#[derive(Eq, PartialEq)]
pub struct ALDevice {
    raw: *mut ALCdevice,
}

unsafe impl Sync for ALDevice {}

unsafe impl Send for ALDevice {}

impl ALDevice {
    #[inline(always)]
    pub unsafe fn raw(&self) -> *mut ALCdevice { self.raw }

    pub fn open(name: Option<&str>) -> ALResult<Arc<ALDevice>> {
        let device = if let Some(name) = name {
            let c_name = try_throw!(CString::new(name));

            unsafe { alcOpenDevice(c_name.as_ptr() as *const ALchar) }
        } else {
            unsafe { alcOpenDevice(ptr::null()) }
        };

        if device.is_null() {
            check_alc_errors!();

            panic!("Could not create OpenAL device");
        }

        Ok(Arc::new(ALDevice { raw: device }))
    }

    pub fn extension_present(&self, extension: &str) -> ALResult<bool> {
        let c_ext = try_throw!(CString::new(extension));

        let res = unsafe { alcIsExtensionPresent(self.raw, c_ext.as_ptr() as *const _) };

        check_alc_errors!();

        Ok(res == ALC_TRUE)
    }

    pub fn name(&self) -> ALResult<Cow<str>> {
        if try_rethrow!(self.extension_present(ALC_ENUMERATE_ALL_EXT_NAME)) {
            self.get_string(ALC_ALL_DEVICES_SPECIFIER)
        } else {
            self.get_string(ALC_DEVICE_SPECIFIER)
        }
    }

    pub fn get_integer(&self, param: ALenum) -> ALResult<ALint> {
        let mut res: ALint = 0;

        unsafe { alcGetIntegerv(self.raw, param, 1, &mut res); }

        check_alc_errors!();

        Ok(res)
    }

    pub fn get_enum(&self, name: &str) -> ALResult<ALenum> {
        let c_str = try_throw!(CString::new(name));

        let res = unsafe { alcGetEnumValue(self.raw, c_str.as_ptr()) };

        check_alc_errors!();

        if res == 0 || res == -1 {
            throw!(ALError::InvalidEnum);
        } else {
            Ok(res)
        }
    }

    pub fn get_string(&self, param: ALenum) -> ALResult<Cow<str>> {
        let c_str = unsafe { alcGetString(self.raw, param) };

        check_alc_errors!();

        Ok(unsafe { CStr::from_ptr(c_str).to_string_lossy() })
    }

    pub fn get_stringi(&self, param: ALenum, i: ALint) -> ALResult<Cow<str>> {
        let c_str = unsafe { alcGetStringiSOFT(self.raw, param, i) };

        check_alc_errors!();

        Ok(unsafe { CStr::from_ptr(c_str).to_string_lossy() })
    }

    /// The OpenAL Enumeration extension allows for multiple strings to be returned for special parameters.
    ///
    /// Supported multistring parameters:
    ///
    /// ```ignore
    /// ALC_ALL_DEVICES_SPECIFIER
    /// ALC_DEVICE_SPECIFIER
    /// ALC_CAPTURE_DEVICE_SPECIFIER
    /// ```
    ///
    /// If the parameter given is not one of those,
    /// `get_multistring` will just return a single element vector with the result of `get_string`
    ///
    /// Note that the `ALL_` params require the `ALC_ENUMERATE_ALL_EXT` extension.
    pub fn get_multistring(&self, param: ALenum) -> ALResult<Vec<Cow<str>>> {
        const MULTISTRING_PARAMS: &'static [ALenum] = &[
            ALC_ALL_DEVICES_SPECIFIER,
            ALC_DEVICE_SPECIFIER,
            ALC_CAPTURE_DEVICE_SPECIFIER];

        if MULTISTRING_PARAMS.contains(&param) {
            let mut c_strs = unsafe { alcGetString(self.raw, param) };

            check_alc_errors!();

            let mut results = Vec::new();

            if !c_strs.is_null() {
                loop {
                    unsafe {
                        let res = CStr::from_ptr(c_strs);

                        results.push(res.to_string_lossy());

                        c_strs = c_strs.offset(res.to_bytes_with_nul().len() as isize);

                        if *c_strs == '\0' as ALchar { break; }
                    }
                }
            }

            Ok(results)
        } else {
            Ok(vec![try_rethrow!(self.get_string(param))])
        }
    }
}

pub trait ALDeviceArc {
    fn create_context(&self) -> ALResult<Arc<ALContext>>;
    fn create_listener(&self) -> ALResult<Arc<ALListener>>;
}

impl ALDeviceArc for Arc<ALDevice> {
    fn create_context(&self) -> ALResult<Arc<ALContext>> {
        ALContext::create_from_device(self.clone())
    }

    fn create_listener(&self) -> ALResult<Arc<ALListener>> {
        Ok(ALListener::new(try_rethrow!(self.create_context())))
    }
}

impl Drop for ALDevice {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { alcCloseDevice(self.raw); }

            ALError::check_alc().unwrap();
        }
    }
}
