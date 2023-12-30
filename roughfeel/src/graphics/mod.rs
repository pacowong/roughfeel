use num_traits::{Float, FromPrimitive};

pub mod drawable;
pub mod drawable_maker;
pub mod drawable_ops;
mod filler;
mod geometry;
pub mod paint;
pub mod points_on_path;
mod render_context;
mod renderer;

pub fn _c<U: Float + FromPrimitive>(inp: f32) -> U {
    U::from(inp).expect("can not parse from f32")
}

pub fn _cc<U: Float + FromPrimitive>(inp: f64) -> U {
    U::from(inp).expect("can not parse from f64")
}
