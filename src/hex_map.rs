use bevy::prelude::*;

use crate::{
    config::HEX_TILE_ANCHOR,
    hex_coords::{axial_to_pixel, AxialCoord},
    util::get_z_index,
};

pub struct HexMap;

impl Plugin for HexMap {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (spawn_grids, setup_hex_grids).chain());
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
                        offset.z - 1000. + get_z_index(pixel.y),
                    ),
                },))
                .id();

            grid.cells[i] = Some(id);
        }
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
