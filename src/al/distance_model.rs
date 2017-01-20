use als::all::*;

use super::error::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ALDistanceModel {
    InverseDistance,
    InverseDistanceClamped,
    LinearDistance,
    LinearDistanceClamped,
    ExponentDistance,
    ExponentDistanceClamped,
}

impl ALDistanceModel {
    pub fn to_alenum(&self) -> ALenum {
        match *self {
            ALDistanceModel::InverseDistance => AL_INVERSE_DISTANCE,
            ALDistanceModel::InverseDistanceClamped => AL_INVERSE_DISTANCE_CLAMPED,
            ALDistanceModel::LinearDistance => AL_LINEAR_DISTANCE,
            ALDistanceModel::LinearDistanceClamped => AL_LINEAR_DISTANCE_CLAMPED,
            ALDistanceModel::ExponentDistance => AL_EXPONENT_DISTANCE,
            ALDistanceModel::ExponentDistanceClamped => AL_EXPONENT_DISTANCE_CLAMPED,
        }
    }

    pub fn from_alenum(model: ALenum) -> ALResult<ALDistanceModel> {
        Ok(match model {
            AL_INVERSE_DISTANCE => ALDistanceModel::InverseDistance,
            AL_INVERSE_DISTANCE_CLAMPED => ALDistanceModel::InverseDistanceClamped,
            AL_LINEAR_DISTANCE => ALDistanceModel::LinearDistance,
            AL_LINEAR_DISTANCE_CLAMPED => ALDistanceModel::LinearDistanceClamped,
            AL_EXPONENT_DISTANCE => ALDistanceModel::ExponentDistance,
            AL_EXPONENT_DISTANCE_CLAMPED => ALDistanceModel::ExponentDistanceClamped,
            _ => throw!(ALError::InvalidEnum)
        })
    }
}