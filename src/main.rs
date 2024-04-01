use bevy::prelude::*;

use crate::animation::{AnimationState, Animator};
use crate::player::{Player, PlayerAnimations, PlayerMovement, PlayerMovementPlugin};

mod player;
mod animation;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("viper.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.67, 25.0), 3, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player { mass: 2.0 },
        PlayerMovement {
            speed: 200.0,
            friction: 0.5,
            ..default()
        },
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            },
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        AnimationState::default(),
        Animator::from(player::create_animation_bank()),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerMovementPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animation::animate_sprite::<PlayerAnimations>)
        .run();
}
