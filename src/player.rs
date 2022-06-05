use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(move_player)
            .with_system(spawn_camera_follow));
    }
}


fn spawn_camera_follow(
    time: Res<Time>,
    actions: Res<Actions>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>)>,
) {
    for mut transform in camera_query.iter_mut() {


        if let Some(mouse) = actions.mouse_right_drag {
            if mouse.is_nan() {
                return;
            }
            transform.translation = transform
                .translation
                .lerp(mouse.extend(0.0), time.delta_seconds() * 1.2);
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
