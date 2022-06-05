use crate::GameState;
use bevy::prelude::*;
use bevy::render::camera::{Camera2d, RenderTarget};

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_movement_actions)
                .with_system(handle_mouse_clicks),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub mouse_right_drag: Option<Vec2>,
}


pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut actions: ResMut<Actions>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {

    let (camera, camera_transform) = q_camera.single();

    let win = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if mouse_input.pressed(MouseButton::Right)|| mouse_input.pressed(MouseButton::Left) {
        if let Some(screen_pos) = win.cursor_position() {
            let window_size = Vec2::new(win.width() as f32, win.height() as f32);

            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();

            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            actions.mouse_right_drag = Some(world_pos.truncate());
        }
        return;
    } else {
        actions.mouse_right_drag = None;
    }
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Up.just_released(&keyboard_input)
        || GameControl::Up.pressed(&keyboard_input)
        || GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Down.just_released(&keyboard_input)
        || GameControl::Down.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input)
            || GameControl::Down.just_released(&keyboard_input)
        {
            if GameControl::Up.pressed(&keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        actions.player_movement = None;
    }
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
