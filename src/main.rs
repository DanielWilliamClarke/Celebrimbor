use bevy::prelude::*;
use crate::player_movement::{Player, PlayerMovement, PlayerMovementPlugin};

pub mod player_movement;

fn setup (
    mut commands: Commands,
    asset_server : Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("viper.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.67, 25.0), 3 ,2, None, None);
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
                index: 1
            },
            ..default()
        }
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerMovementPlugin)
        .add_systems(Startup, setup)
        .run();
}
