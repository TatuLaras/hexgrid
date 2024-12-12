const MOVING_DOWN_ROW: usize = 0;
const MOVING_HORIZONTALLY_ROW: usize = 1;
const MOVING_UP_ROW: usize = 2;

use bevy::prelude::*;

use crate::{
    player_movement::{Movement, MovementState},
    Player,
};

pub struct PlayerVisuals;

impl Plugin for PlayerVisuals {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut Sprite,
            &Movement,
            &MovementAnimation,
        ),
        With<Player>,
    >,
) {
    for (mut timer, mut sprite, movement, movement_animation) in &mut query {
        // Pick correct animation to play by movement state
        let animation = match movement.state {
            MovementState::Idle => &movement_animation.idle,
            MovementState::Up => &movement_animation.up,
            MovementState::Down => &movement_animation.down,
            MovementState::Right => &movement_animation.right,
            MovementState::Left => &movement_animation.right,
        };

        // Flip the sprite horizontally if going left
        sprite.flip_x = match movement.state {
            MovementState::Left => true,
            _ => false,
        };

        // Get texture atlas
        let Some(atlas) = &mut sprite.texture_atlas else { 
            eprintln!("Sprite texture atlas missing in player_visuals::animate_sprite");
            return; 
        };

        // If current index outside the bounds of the animation fix that
        if atlas.index < animation.from_i || atlas.index > animation.to_i {
            atlas.index = animation.from_i;
        }

        timer.tick(time.delta());

        if timer.just_finished() {
            atlas.index = if atlas.index == animation.to_i {
                animation.from_i
            } else { 
                atlas.index + 1 
            };
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct MovementAnimation {
    pub up: AnimationIndices,
    pub down: AnimationIndices,
    pub right: AnimationIndices,
    pub idle: AnimationIndices,
}

pub struct AnimationIndices {
    pub from_i: usize,
    pub to_i: usize,
}
