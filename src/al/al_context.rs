use als::all::*;

use std::ptr;
use std::sync::Arc;

use super::al_error::*;
use super::al_device::*;

pub struct ALContext {
    raw: *mut ALCcontext,
    device: Arc<ALDevice>,
}

impl ALContext {
    #[inline(always)]
    pub unsafe fn raw(&self) -> *mut ALCcontext { self.raw }

    pub fn create_from_device(device: Arc<ALDevice>) -> ALResult<Arc<ALContext>> {
        let ctx = unsafe { alcCreateContext(device.raw(), ptr::null()) };

        if ctx.is_null() {
            check_alc_errors!();

            panic!("Could not create OpenAL context");
        }

        Ok(Arc::new(ALContext { raw: ctx, device: device }))
    }

    pub fn device(&self) -> Arc<ALDevice> { self.device.clone() }

    pub fn make_current(&self) -> ALResult<()> {
        if ALC_TRUE != unsafe { alcMakeContextCurrent(self.raw) } {
            check_alc_errors!();

            panic!("Could not make context current");
        }

        Ok(())
    }

    #[inline]
    pub fn suspend(&self) -> ALResult<()> {
        unsafe { alcSuspendContext(self.raw); }

        check_alc_errors!();

        Ok(())
    }

    #[inline]
    pub fn process(&self) -> ALResult<()> {
        unsafe { alcProcessContext(self.raw); }

        check_alc_errors!();

        Ok(())
    }
}

impl Drop for ALContext {
    fn drop(&mut self) {
        unsafe {
            // Stop using this context before destroying it
            alcMakeContextCurrent(ptr::null_mut());
            alcDestroyContext(self.raw);
        }

        ALError::check_alc().unwrap();
    }
}