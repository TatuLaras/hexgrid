use std::f32::consts::PI;

use bevy::prelude::*;

pub struct PlayerMovement;

impl Plugin for PlayerMovement {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_movement);
    }
}

pub fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Movement)>,
) {
    let left: f32 = ((keyboard_input.pressed(KeyCode::KeyA) as u8) as f32) * -1.0;
    let right: f32 = ((keyboard_input.pressed(KeyCode::KeyD) as u8) as f32) * 1.0;
    let up: f32 = ((keyboard_input.pressed(KeyCode::KeyW) as u8) as f32) * 1.0;
    let down: f32 = ((keyboard_input.pressed(KeyCode::KeyS) as u8) as f32) * -1.0;

    let shift: bool = keyboard_input.pressed(KeyCode::ShiftLeft);

    let direction = Vec2 {
        x: left + right,
        y: up + down,
    };
    const RIGHT: Vec2 = Vec2 { x: 1., y: 0. };

    let direction = direction.normalize_or_zero();

    for (mut transform, mut movement) in &mut query {
        let speed = if shift {
            movement.run_speed
        } else {
            movement.speed
        };

        // Calculate an index into our movement states (moving up, down etc.)
        //  based on the angle of the vector
        movement.state = if direction.length_squared() > 0. {
            ((direction.angle_to(RIGHT) / PI * 4. + 4.).round() as usize).into()
        } else {
            MovementState::Idle
        };

        transform.translation.x += direction.x * speed;
        // Vertical speed needs to be a bit less than horizontal due to the projection
        transform.translation.y += direction.y * speed * 0.7;
    }
}

#[derive(Debug)]
pub enum MovementState {
    Idle,
    Up,
    Down,
    Left,
    Right,
}

impl From<usize> for MovementState {
    fn from(value: usize) -> Self {
        //  NOTE: This can be extended to support 8 directions
        match value {
            0 => MovementState::Left,
            1 => MovementState::Up,
            2 => MovementState::Up,
            3 => MovementState::Up,
            4 => MovementState::Right,
            5 => MovementState::Down,
            6 => MovementState::Down,
            7 => MovementState::Down,
            _ => MovementState::Idle,
        }
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub run_speed: f32,
    pub state: MovementState,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 1.,
            run_speed: 1.,
            state: MovementState::Idle,
        }
    }
}
