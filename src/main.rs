use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor, window::PrimaryWindow};
use character_animation::{CharacterAnimation, MovementAnimation};
use debug_text::{update_debug_text, DebugTextElement};
use game_camera::{FollowsPlayer, GameCamera};
use hex_map::HexMap;
use player_movement::{Movement, PlayerMovement};
use util::get_z_index;

mod character_animation;
mod config;
mod debug_text;
mod game_camera;
mod hex_coords;
mod hex_map;
mod player_movement;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((HexMap, PlayerMovement, GameCamera, CharacterAnimation))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_debug_text, follow_cursor, world_space_cursor),
        )
        .insert_resource(MouseCursorWorldCoords(Vec2 { x: 0., y: 0. }))
        .insert_resource(DebugTextTimer(Timer::from_seconds(
            6.0,
            TimerMode::Repeating,
        )))
        .insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
        .insert_resource(DebugText {
            text: "".into(),
            has_changed: false,
        })
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 2D camera
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 200.0,
        },
        far: 2000.,
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((
        Camera2d,
        projection,
        Transform::from_xyz(0., 0., 0.),
        MainCamera,
        FollowsPlayer { follow_speed: 8. },
    ));

    let cross_sprite = Sprite::from_image(asset_server.load("debug_cross.png"));

    let mut box_sprite = Sprite::from_image(asset_server.load("debug_box.png"));
    box_sprite.anchor = Anchor::BottomCenter;

    // You can use this to visualize pivot of sprite

    commands.spawn((
        box_sprite.clone(),
        Transform::from_xyz(0., 0., 0.),
        FollowsCursor,
    ));

    // A cross at (0, 0) to see where the origo is
    commands.spawn((cross_sprite, Transform::from_xyz(0., 0., 0.)));

    // Static objects
    commands.spawn((
        box_sprite,
        Transform::from_xyz(300., 273., get_z_index(273.)),
    ));

    // Player
    let texture = asset_server.load("debug_bubble_guy.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut player_sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    );

    player_sprite.anchor = Anchor::BottomCenter;

    commands.spawn((
        Player,
        Movement {
            speed: 2.,
            run_speed: 3.,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 1.),
        player_sprite,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        MovementAnimation {
            idle: AnimationIndices { from_i: 0, to_i: 0 },
            down: AnimationIndices { from_i: 1, to_i: 2 },
            right: AnimationIndices { from_i: 4, to_i: 5 },
            up: AnimationIndices { from_i: 7, to_i: 8 },
        },
    ));

    // Debug text element
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        DebugTextElement,
    ));
}
fn world_space_cursor(
    mut mycoords: ResMut<MouseCursorWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

fn follow_cursor(
    mouse_coords: Res<MouseCursorWorldCoords>,
    mut query: Query<&mut Transform, With<FollowsCursor>>,
) {
    for mut transform in &mut query {
        transform.translation = Vec3 {
            x: mouse_coords.0.x,
            y: mouse_coords.0.y,
            z: get_z_index(mouse_coords.0.y),
        };
    }
}

#[derive(Resource, Default)]
struct MouseCursorWorldCoords(Vec2);

#[derive(Component)]
struct FollowsCursor;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct DebugTextTimer(Timer);

#[derive(Resource)]
pub struct DebugText {
    text: String,
    has_changed: bool,
}

impl DebugText {
    pub fn set(&mut self, str: &str) {
        self.text = str.into();
        self.has_changed = true;
    }

    pub fn new_text(&mut self) -> Option<String> {
        if !self.has_changed {
            return None;
        }

        self.has_changed = false;
        return Some(self.text.clone());
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationIndices {
    pub from_i: usize,
    pub to_i: usize,
}
