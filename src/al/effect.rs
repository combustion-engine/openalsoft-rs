use als::all::*;

use std::mem;
use std::sync::Arc;

use super::error::*;
use super::listener::*;
use super::effects::*;

use super::ALObject;

pub struct ALEffect(ALuint, Arc<ALListener>);

impl_simple_alobject!(simple ALEffect, alIsEffect);

impl ALEffect {
    pub fn new(listener: Arc<ALListener>) -> ALResult<Arc<ALEffect>> {
        let mut effect = 0;

        unsafe { alGenEffects(1, &mut effect); }

        check_al_errors!();

        Ok(Arc::new(ALEffect(effect, listener)))
    }
}