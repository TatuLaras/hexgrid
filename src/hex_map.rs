use std::borrow::BorrowMut;

use bevy::app::Plugin;
use bevy::prelude::*;

use crate::{
    config::HEX_TILE_ANCHOR,
    hex_coords::{axial_to_pixel, pixel_to_axial, AxialCoord},
    MouseCursorWorldCoords,
};

pub struct HexMap;

impl Plugin for HexMap {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (spawn_grids, setup_hex_grids).chain());
        app.add_systems(Update, (add_tree, plant_tree));
    }
}

fn spawn_grids(mut commands: Commands) {
    commands.spawn((
        HexGrid::from_size(16, 24),
        Transform::from_xyz(100., 40., 0.),
    ));

    // let mut grid = HexGrid::from_size(16, 24);
    // grid.sprite = "cells/debug_top_layer_w128.png".into();
    //
    // commands.spawn((grid, Transform::from_xyz(0., HEX_WIDTH, 0.)));
}

fn setup_hex_grids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grids: Query<(&mut HexGrid, &Transform)>,
) {
    for (mut grid, transform) in &mut grids {
        for i in 0..grid.cells.len() {
            let offset = transform.translation;

            let q: i32 = i as i32 % grid.width as i32;
            let r: i32 = i as i32 / grid.width as i32;

            let pixel = axial_to_pixel(AxialCoord { q, r });

            let mut sprite = Sprite::from_image(asset_server.load(grid.sprite.clone()));
            sprite.anchor = HEX_TILE_ANCHOR;

            let id = commands
                .spawn((TileBundle {
                    sprite,
                    transform: Transform::from_xyz(
                        offset.x + pixel.x,
                        offset.y + pixel.y,
                        offset.z - pixel.y * 0.01,
                    ),
                },))
                .id();

            grid.cells[i] = Some(id);
        }
    }
    println!("{:?}", grids);
}

fn plant_tree(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<MouseCursorWorldCoords>,
    mut commands: Commands,
    query: Query<(&HexGrid, &Transform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (grid, transform) in &query {
            let offset = Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            };

            let axial = pixel_to_axial(mouse_coords.0 - offset);

            let index: i32 = axial.r * grid.width as i32 + axial.q;

            let index = if index < 0 || index as usize >= grid.cells.len() {
                continue;
            } else {
                index as usize
            };

            let Some(cell) = grid.cells[index] else { continue };

            commands.entity(cell).insert(HasTree);
        }
    }
}

fn add_tree(mut sprites: Query<&mut Sprite, Added<HasTree>>, asset_server: Res<AssetServer>) {
    for mut sprite in &mut sprites {
        let handle = sprite.image.borrow_mut();
        *handle = asset_server.load("cells/debug_tree_w128.png");
    }
}

#[derive(Bundle)]
struct TileBundle {
    sprite: Sprite,
    transform: Transform,
}

#[derive(Component, Debug)]
struct HexGrid {
    pub width: u16,
    pub cells: Vec<Option<Entity>>,
    pub sprite: String,
}

impl Default for HexGrid {
    fn default() -> Self {
        Self {
            width: 8,
            cells: vec![None; 0],
            sprite: "cells/debug_w128.png".into(),
        }
    }
}

#[derive(Component)]
struct HasTree;

impl HexGrid {
    pub fn from_size(width: u16, height: u16) -> Self {
        let cell_count: usize = (width * height).into();

        Self {
            width,
            cells: vec![None; cell_count],
            ..Default::default()
        }
    }
}
