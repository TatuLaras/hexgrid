use bevy::{math::Vec2, sprite::Anchor};

pub const HEX_WIDTH: f32 = 128.;
pub const HEX_TILE_ANCHOR: Anchor = Anchor::Custom(Vec2 { x: 0., y: -0.25 });
