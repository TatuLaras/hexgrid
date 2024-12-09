use bevy::math::Vec2;

use crate::config::HEX_WIDTH;

#[derive(Debug)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

#[derive(Debug)]
pub struct AxialCoordFloating {
    pub q: f32,
    pub r: f32,
}

#[derive(Debug)]
pub struct CubeCoord {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

#[derive(Debug)]
pub struct CubeCoordFloating {
    pub q: f32,
    pub r: f32,
    pub s: f32,
}

impl From<CubeCoord> for AxialCoord {
    fn from(value: CubeCoord) -> Self {
        return Self {
            q: value.q,
            r: value.r,
        };
    }
}

impl From<CubeCoordFloating> for AxialCoordFloating {
    fn from(value: CubeCoordFloating) -> Self {
        return Self {
            q: value.q,
            r: value.r,
        };
    }
}

impl From<AxialCoord> for CubeCoord {
    fn from(value: AxialCoord) -> Self {
        return Self {
            q: value.q,
            r: value.r,
            s: -value.q - value.r,
        };
    }
}

impl From<AxialCoordFloating> for CubeCoordFloating {
    fn from(value: AxialCoordFloating) -> Self {
        return Self {
            q: value.q,
            r: value.r,
            s: -value.q - value.r,
        };
    }
}

pub fn axial_to_pixel(axial: AxialCoord) -> Vec2 {
    Vec2 {
        x: HEX_WIDTH * 0.75 * axial.q as f32,
        y: HEX_WIDTH * (0.25 * axial.q as f32 + axial.r as f32 * 0.5),
    }
}

// function pixel_to_flat_hex(point):
//     var q = ( 2./3 * point.x                        ) / size
//     var r = (-1./3 * point.x  +  sqrt(3)/3 * point.y) / size
//     return axial_round(Hex(q, r))
pub fn pixel_to_axial(pixel: Vec2) -> AxialCoord {
    axial_round(AxialCoordFloating {
        q: (4. * pixel.x) / (3. * HEX_WIDTH as f32),
        r: (6. * pixel.y - 2. * pixel.x) / (3. * HEX_WIDTH),
    })
}

pub fn cube_round(float_cube: CubeCoordFloating) -> CubeCoord {
    let mut q = (float_cube.q).round() as i32;
    let mut r = (float_cube.r).round() as i32;
    let mut s = (float_cube.s).round() as i32;

    let q_diff = (q as f32 - float_cube.q).abs();
    let r_diff = (r as f32 - float_cube.r).abs();
    let s_diff = (s as f32 - float_cube.s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s
    } else {
        s = -q - r
    }

    return CubeCoord { q, r, s };
}

pub fn axial_round(float_axial: AxialCoordFloating) -> AxialCoord {
    cube_round(float_axial.into()).into()
}
