use std::fmt::Display;
use std::ops::MulAssign;

use num_traits::{Float, FromPrimitive};

use euclid::{Trig, default::Point2D};

use super::drawable_ops::{OpSet};

use super::drawable::{Drawable, DrawOptions};
use super::renderer;

/*
struct RoughRenderContext {}

impl RoughRenderContext {
    fn new() -> Self {
        RoughRenderContext { }
    }

    fn d<T, F, OutputDrawable: Drawable>(&self, name: T, op_sets: &[OpSet<F>], options: &Option<DrawOptions>) -> OutputDrawable
    where
        T: Into<String>,
        F: Float + Trig + FromPrimitive,
    {
        OutputDrawable::draw {
            shape: name.into(),
            options: options
                .clone()
                .unwrap_or_else(|| self.default_options.clone()),
            sets: Vec::from_iter(op_sets.iter().cloned()),
        }
    }

    pub fn line<F, OutputDrawable: Drawable>(&self, x1: F, y1: F, x2: F, y2: F, options: &Option<DrawOptions>) -> OutputDrawable
    where
        F: Float + Trig + FromPrimitive,
    {
        self.d(
            "line",
            &[renderer::line(
                x1,
                y1,
                x2,
                y2,
                &mut options
                    .clone()
                    .unwrap_or_else(|| self.default_options.clone()),
            )],
            options,
        )
    }
}
*/

pub trait RoughlyCanvas<F: Trig + Float + FromPrimitive + MulAssign + Display, D: Drawable<OpSet<F> > > {
    fn draw_line(
        &self,
        x1: F,
        y1: F,
        x2: F,
        y2: F,
        options: DrawOptions
    );

    fn draw_rectangle(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: DrawOptions
    );

    fn draw_ellipse(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: DrawOptions
    );

    fn draw_circle(
        &self,
        x: F,
        y: F,
        diameter: F,
        options: DrawOptions
    );

    fn draw_linear_path(
        &self,
        points: &[Point2D<F>],
        close: bool,
        options: DrawOptions
    );

    fn draw_polygon(
        &self,
        points: &[Point2D<F>],
    );

    fn draw_arc(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        start: F,
        stop: F,
        closed: bool,
    );

    fn draw_bezier_quadratic(
        &self,
        start: Point2D<F>,
        cp: Point2D<F>,
        end: Point2D<F>,
    );

    fn draw_bezier_cubic(
        &self,
        start: Point2D<F>,
        cp1: Point2D<F>,
        cp2: Point2D<F>,
        end: Point2D<F>,
    );

    fn draw_curve(
        &self,
        points: &[Point2D<F>],
    );

    fn draw_path(
        &self,
        svg_path: String,
    );
}
