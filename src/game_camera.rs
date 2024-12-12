use bevy::prelude::*;

use crate::Player;

pub struct GameCamera;

impl Plugin for GameCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_position);
    }
}

fn update_camera_position(
    mut player_followers: Query<(&mut Transform, &FollowsPlayer)>,
    player: Query<&Transform, (With<Player>, Without<FollowsPlayer>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player.get_single() else { 
        eprintln!("system update_camera_position: did not run, 
            multiple entities with component Player even though there should only be one");
        return;
    };

    // Smoothly lerp the camera position towards the player
    for (mut transform, follows_player) in &mut player_followers {
        let camera_pos = transform.translation.truncate();
        let player_pos = player_transform.translation.truncate();

        transform.translation = camera_pos
            .lerp(player_pos, time.delta_secs() * follows_player.follow_speed)
            .extend(transform.translation.z);
    }
}

#[derive(Component)]
pub struct FollowsPlayer {
    pub follow_speed: f32,
}
