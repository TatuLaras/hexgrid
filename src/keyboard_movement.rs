use bevy::prelude::*;

use crate::DebugText;

#[derive(Component)]
pub struct KeyboardMovement {
    pub speed: f32,
}

pub fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_text: ResMut<DebugText>,
    mut query: Query<(&mut Transform, &KeyboardMovement)>,
) {
    let left: f32 = ((keyboard_input.pressed(KeyCode::KeyA) as u8) as f32) * -1.0;
    let right: f32 = ((keyboard_input.pressed(KeyCode::KeyD) as u8) as f32) * 1.0;
    let up: f32 = ((keyboard_input.pressed(KeyCode::KeyW) as u8) as f32) * 1.0;
    let down: f32 = ((keyboard_input.pressed(KeyCode::KeyS) as u8) as f32) * -1.0;

    let shift: f32 = 1. + ((keyboard_input.pressed(KeyCode::ShiftLeft) as u8) as f32) * 1.0;

    let direction = Vec2 {
        x: left + right,
        y: up + down,
    };

    let mut direction = direction.normalize_or_zero();
    direction *= shift;

    for (mut transform, keyboard_movement) in &mut query {
        transform.translation.x += direction.x * keyboard_movement.speed;
        transform.translation.y += direction.y * keyboard_movement.speed;
    }
}
