use nalgebra::{Vector};
use nalgebra_glm::RealNumber;
use num_traits::{Float, FromPrimitive};

pub mod drawable;
pub mod drawable_maker;
pub mod drawable_ops;
mod filler;
mod geometry;
pub mod paint;
pub mod points_on_path;
pub mod render_context;
pub mod renderer;

use std::{f32, f64};

pub fn _c<U: RealNumber>(inp: f32) -> U {
    U::from_f32(inp).expect("can not parse from f32")
}

pub fn _cc<U: RealNumber>(inp: f64) -> U {
    U::from_f64(inp).expect("can not parse from f64")
}

pub fn _to_u64<U: RealNumber>(inp: U) -> u64 {
    _to_f64(inp) as u64
}

pub fn _to_f64<U: RealNumber>(inp: U) -> f64 {
    nalgebra::try_convert(inp).unwrap()
}

pub fn _to_f32<U: RealNumber>(inp: U) -> f32 {
    nalgebra::try_convert(inp).unwrap() as f32
}
