use als::all::*;

use std::ptr;
use std::marker::PhantomData;

use super::al_error::*;
use super::al_device::*;

pub struct ALContext<'a> {
    raw: *mut ALCcontext,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ALContext<'a> {
    #[inline(always)]
    pub unsafe fn raw(&self) -> *mut ALCcontext { self.raw }

    pub fn create_from_device<'b: 'a>(device: &mut ALDevice<'b>) -> ALResult<ALContext<'a>> {
        let ctx = unsafe { alcCreateContext(device.raw(), ptr::null()) };

        if ctx.is_null() {
            check_alc_errors!();

            panic!("Could not create OpenAL context");
        }

        Ok(ALContext { raw: ctx, _marker: PhantomData })
    }

    pub fn make_current(&mut self) -> ALResult<()> {
        if ALC_TRUE != unsafe { alcMakeContextCurrent(self.raw) } {
            check_alc_errors!();

            panic!("Could not make context current");
        }

        Ok(())
    }

    #[inline]
    pub fn suspend(&mut self) -> ALResult<()> {
        unsafe { alcSuspendContext(self.raw); }

        check_alc_errors!();

        Ok(())
    }

    #[inline]
    pub fn process(&mut self) -> ALResult<()> {
        unsafe { alcProcessContext(self.raw); }

        check_alc_errors!();

        Ok(())
    }
}

impl<'a> Drop for ALContext<'a> {
    fn drop(&mut self) {
        unsafe {
            alcMakeContextCurrent(ptr::null_mut());
            alcDestroyContext(self.raw);
        }

        ALError::check_alc().unwrap();
    }
}