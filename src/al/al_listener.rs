use als::all::*;

use std::ptr;
use std::ops::Deref;
use std::sync::Arc;

use nalgebra::*;

use super::al_error::*;
use super::al_device::*;
use super::al_context::*;

pub struct ALListener {
    device: Arc<ALDevice>,
    context: Arc<ALContext>,
}

impl ALListener {
    pub fn new(device: Arc<ALDevice>, context: Arc<ALContext>) -> Arc<ALListener> {
        Arc::new(ALListener { device: device, context: context })
    }

    #[inline(always)]
    pub fn device(&self) -> Arc<ALDevice> { self.device.clone() }
}

impl Deref for ALListener {
    type Target = ALContext;

    #[inline(always)]
    fn deref(&self) -> &ALContext { &self.context }
}

impl ALListener {
    pub fn set_gain(&self, gain: ALfloat) -> ALResult<()> {
        unsafe { alListenerf(AL_GAIN, gain); }

        check_al_errors!();

        Ok(())
    }

    pub fn get_gain(&self) -> ALResult<ALfloat> {
        let mut gain = 0.0;

        unsafe { alGetListenerf(AL_GAIN, &mut gain); }

        check_al_errors!();

        Ok(gain)
    }

    pub fn set_velocity(&self, velocity: Vector3<f32>) -> ALResult<()> {
        unsafe { alListener3f(AL_VELOCITY, velocity.x as ALfloat, velocity.y as ALfloat, velocity.z as ALfloat); }

        check_al_errors!();

        Ok(())
    }

    pub fn set_position(&self, position: Point3<f32>) -> ALResult<()> {
        unsafe { alListener3f(AL_POSITION, position.x as ALfloat, position.y as ALfloat, position.z as ALfloat); }

        check_al_errors!();

        Ok(())
    }

    pub fn get_velocity(&self) -> ALResult<Vector3<f32>> {
        let mut velocity: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        unsafe { alGetListenerfv(AL_VELOCITY, &mut velocity as *mut _ as *mut ALfloat); }

        check_al_errors!();

        Ok(velocity)
    }

    pub fn get_position(&self) -> ALResult<Point3<f32>> {
        let mut position: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

        unsafe { alGetListenerfv(AL_POSITION, &mut position as *mut _ as *mut ALfloat); }

        check_al_errors!();

        Ok(position)
    }

    pub fn set_orientation(&self, at: Vector3<f32>, up: Option<Vector3<f32>>) -> ALResult<()> {
        let up = up.unwrap_or(Vector3::new(0.0, 1.0, 0.0));

        //Combine them into a single 6-element vector so it can be passed by memory location
        let at_up = Vector6::new(at.x, at.y, at.z, up.x, up.y, up.z);

        unsafe { alListenerfv(AL_ORIENTATION, &at_up as *const _ as *const ALfloat); }

        check_al_errors!();

        Ok(())
    }

    pub fn get_orientation(&self) -> ALResult<(Vector3<f32>, Vector3<f32>)> {
        //Same as above
        let mut at_up = Vector6::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        unsafe { alGetListenerfv(AL_ORIENTATION, &mut at_up as *mut _ as *mut ALfloat); }

        check_al_errors!();

        //Get the individual halves
        let at = Vector3::new(at_up.x, at_up.y, at_up.z);
        let up = Vector3::new(at_up.w, at_up.a, at_up.b);

        Ok((at, up))
    }
}