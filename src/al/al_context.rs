use als::all::*;

use std::ptr;
use std::sync::Arc;
use std::cell::Cell;
use std::ops::Deref;

use super::al_error::*;
use super::al_device::*;
use super::al_listener::*;

pub struct ALContext {
    raw: *mut ALCcontext,
    device: Arc<ALDevice>,
    thread_local: Cell<bool>,
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

        let ctx = Arc::new(ALContext { raw: ctx, device: device, thread_local: Cell::new(false) });

        ctx.make_current()?;

        Ok(ctx)
    }

    pub fn device(&self) -> Arc<ALDevice> { self.device.clone() }

    pub fn make_current(&self) -> ALResult<()> {
        if ALC_TRUE != unsafe { alcMakeContextCurrent(self.raw) } {
            check_alc_errors!();

            panic!("Could not make context current");
        }

        self.thread_local.set(false);

        Ok(())
    }

    pub fn set_thread_context(&self) -> ALResult<()> {
        if ALC_TRUE != unsafe { alcSetThreadContext(self.raw) } {
            check_alc_errors!();

            panic!("Could not make context current");
        }

        self.thread_local.set(true);

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

impl Deref for ALContext {
    type Target = Arc<ALDevice>;

    #[inline(always)]
    fn deref(&self) -> &Arc<ALDevice> { &self.device }
}

pub trait ALContextArc {
    fn create_listener(&self) -> Arc<ALListener>;
}

impl ALContextArc for Arc<ALContext> {
    fn create_listener(&self) -> Arc<ALListener> {
        ALListener::new(self.clone())
    }
}

impl Drop for ALContext {
    fn drop(&mut self) {
        unsafe {
            // Stop using this context before destroying it
            if self.thread_local.get() {
                alcSetThreadContext(ptr::null_mut());
            } else {
                alcMakeContextCurrent(ptr::null_mut());
            }

            alcDestroyContext(self.raw);
        }

        ALError::check_alc().unwrap();
    }
}