use bevy::prelude::*;

// #[derive(Component)]
// struct Person;

fn setup (
    mut commands: Commands,
    asset_server : Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("viper.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.67, 25.0), 3 ,2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1
            },
            ..default()
        }
    );
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), PlayerPlugin))
        .run();
}
