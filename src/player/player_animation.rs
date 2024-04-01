use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use benimator::FrameRate;

use PlayerAnimations::*;

use crate::animation::{Animation, AnimationBank};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum PlayerAnimations {
    IDLE,
    UP,
    DOWN,
}

impl Display for PlayerAnimations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IDLE => write!(f, "IDLE"),
            UP => write!(f, "UP"),
            DOWN => write!(f, "DOWN")
        }
    }
}

pub fn create_animation_bank() -> AnimationBank<PlayerAnimations> {
    let mut bank = HashMap::new();

    bank.insert(
        IDLE,
        Animation(benimator::Animation::from_indices(
            0..=0,
            FrameRate::from_fps(1.0),
        )),
    );
    bank.insert(
        UP,
        Animation(benimator::Animation::from_indices(
            3..=5,
            FrameRate::from_fps(1.2),
        ).once()),
    );
    bank.insert(
        DOWN,
        Animation(benimator::Animation::from_indices(
            0..=2,
            FrameRate::from_fps(1.2),
        ).once()),
    );

    return AnimationBank {
        bank,
        default_animation: IDLE,
    };
}