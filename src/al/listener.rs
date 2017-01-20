use als::all::*;

use std::ops::Deref;
use std::sync::Arc;

use nalgebra::*;

use super::error::*;
use super::state::*;
use super::device::*;
use super::context::*;
use super::source::*;
use super::source_3d::*;
use super::distance_model::*;

pub struct ALListener {
    context: Arc<ALContext>,
}

impl Deref for ALListener {
    type Target = ALContext;

    #[inline(always)]
    fn deref(&self) -> &ALContext { &self.context }
}

impl ALListener {
    pub fn new(context: Arc<ALContext>) -> Arc<ALListener> {
        Arc::new(ALListener { context: context })
    }

    #[inline(always)]
    pub fn device(&self) -> Arc<ALDevice> { self.context.device() }

    pub fn set_distance_model(&self, model: Option<ALDistanceModel>) -> ALResult<()> {
        unsafe { alDistanceModel(model.map_or(AL_NONE, |m| m.to_alenum())); }

        check_al_errors!();

        Ok(())
    }

    #[inline]
    pub fn get_distance_model(&self) -> ALResult<ALDistanceModel> {
        ALDistanceModel::from_alenum(try_rethrow!(ALState::get_integer(AL_DISTANCE_MODEL)))
    }

    pub fn set_doppler_factor(&self, factor: ALfloat) -> ALResult<()> {
        unsafe { alDopplerFactor(factor); }

        check_al_errors!();

        Ok(())
    }

    #[inline]
    pub fn get_doppler_factor(&self) -> ALResult<ALfloat> {
        ALState::get_float(AL_DOPPLER_FACTOR)
    }

    pub fn set_speed_of_sound(&self, value: ALfloat) -> ALResult<()> {
        unsafe { alSpeedOfSound(value); }

        check_al_errors!();

        Ok(())
    }

    #[inline]
    pub fn get_speed_of_sound(&self) -> ALResult<ALfloat> {
        ALState::get_float(AL_SPEED_OF_SOUND)
    }

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

/// Allows easy creation of sources from an `Arc<ALListener>`
pub trait ALListenerArc {
    /// Create a new `ALSource`
    fn new_source(&self) -> ALResult<Arc<ALSource>>;
    /// Create a new `ALSource3D`
    fn new_3d_source(&self) -> ALResult<Arc<ALSource3D>>;
}

impl ALListenerArc for Arc<ALListener> {
    #[inline(always)]
    fn new_source(&self) -> ALResult<Arc<ALSource>> {
        ALSource::new(self.clone())
    }

    #[inline(always)]
    fn new_3d_source(&self) -> ALResult<Arc<ALSource3D>> {
        ALSource3D::new(self.clone())
    }
}