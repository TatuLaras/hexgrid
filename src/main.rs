use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor, window::PrimaryWindow};
use config::HEX_TILE_ANCHOR;
use debug_text::{update_debug_text, DebugTextElement};
use hex_coords::pixel_to_axial;
use hex_map::HexMap;
use keyboard_movement::{keyboard_movement, KeyboardMovement};

pub mod config;
pub mod debug_text;
pub mod hex_coords;
pub mod hex_map;
pub mod keyboard_movement;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MouseCursorWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct FollowsCursor;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 200.0,
        },
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((
        Camera2d,
        projection,
        Transform::from_xyz(200.0, 200.0, 0.0),
        KeyboardMovement { speed: 6.0 },
        MainCamera,
    ));

    let cross_sprite = Sprite::from_image(asset_server.load("debug_cross.png"));

    commands.spawn((
        cross_sprite.clone(),
        Transform::from_xyz(0., 0., 0.),
        FollowsCursor,
    ));

    commands.spawn((cross_sprite, Transform::from_xyz(0., 0., 0.)));

    commands.spawn((
        // Here we are able to call the `From` method instead of creating a new `TextSection`.
        // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
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
            z: transform.translation.z,
        };
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(HexMap)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_movement,
                update_debug_text,
                follow_cursor,
                world_space_cursor,
            ),
        )
        .insert_resource(MouseCursorWorldCoords(Vec2 { x: 0., y: 0. }))
        .insert_resource(DebugTextTimer(Timer::from_seconds(
            6.0,
            TimerMode::Repeating,
        )))
        .insert_resource(DebugText {
            text: "".into(),
            has_changed: false,
        })
        .run();
}
