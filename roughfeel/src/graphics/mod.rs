use num_traits::{Float, FromPrimitive};

mod render_context;
pub mod drawable;
pub mod paint;
pub mod drawable_ops;
pub mod drawable_maker;
pub mod points_on_path;
mod renderer;
mod geometry;
mod filler;

pub fn _c<U: Float + FromPrimitive>(inp: f32) -> U {
    U::from(inp).expect("can not parse from f32")
}

pub fn _cc<U: Float + FromPrimitive>(inp: f64) -> U {
    U::from(inp).expect("can not parse from f64")
}