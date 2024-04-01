use std::collections::HashMap;
use std::hash::Hash;

use bevy::prelude::*;

#[derive(Deref, Clone)]
pub struct Animation(pub benimator::Animation);

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

#[derive(Clone)]
pub struct AnimationBank<T> {
    pub bank: HashMap<T, Animation>,
    pub default_animation: T,
}

#[derive(Component, Clone)]
pub struct Animator<T> {
    pub animation_bank: AnimationBank<T>,
    pub current_animation: T,
    pub last_animation: T,
}

impl<T> From<AnimationBank<T>> for Animator<T>
    where
        T: Clone
{
    fn from(animations: AnimationBank<T>) -> Self {
        Animator {
            animation_bank: animations.clone(),
            current_animation: animations.clone().default_animation,
            last_animation: animations.clone().default_animation,
        }
    }
}

pub fn animate_sprite<T>(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut Animator<T>, &mut AnimationState)>,
)
    where
        T: std::fmt::Display + Clone + Hash + Eq + Send + Sync + 'static
{
    for (mut texture, mut animator, mut state) in query.iter_mut() {
        let animation = &animator.animation_bank.bank[&animator.current_animation];

        // Update the state
        state.update(&animation, time.delta());

        // Update the texture atlas
        texture.index = state.frame_index();

        animator.last_animation = animator.current_animation.clone();
    }
}



