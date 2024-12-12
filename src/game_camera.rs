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

    for (mut transform, follows_player) in &mut player_followers {
        transform.translation.x = transform.translation.x.lerp(
            player_transform.translation.x,
            time.delta_secs() * follows_player.follow_speed,
        );
        transform.translation.y = transform.translation.y.lerp(
            player_transform.translation.y,
            time.delta_secs() * follows_player.follow_speed,
        );
    }
}

#[derive(Component)]
pub struct FollowsPlayer {
    pub follow_speed: f32,
}
