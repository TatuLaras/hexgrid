use bevy::prelude::*;

use crate::{DebugText, DebugTextTimer};

#[derive(Component)]
pub struct DebugTextElement;

pub fn update_debug_text(
    mut debug_text: ResMut<DebugText>,
    mut timer: ResMut<DebugTextTimer>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<DebugTextElement>>,
) {
    let timer = timer.0.tick(time.delta());

    for mut text in &mut query {
        if timer.just_finished() {
            text.0 = "".into();
        }

        let Some(new_text) = debug_text.new_text() else { continue };

        text.0 = new_text.clone();
    }
}
