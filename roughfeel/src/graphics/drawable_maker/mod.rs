pub mod generator;

use crate::graphics::drawable::{DrawOptions, Drawable, OpSetTrait};
use nalgebra::Point2;
use nalgebra_glm::RealNumber;
use std::fmt::Display;
use std::marker::PhantomData;

// Data types
pub struct Generator<OpSetT: OpSetTrait> {
    default_options: DrawOptions,
    phantom_data_opsett: PhantomData<OpSetT>,
}

// Traits
pub trait RoughlyDrawableMakable<F: RealNumber + Display, OpSetT, OutputDrawable>
where
    OpSetT: OpSetTrait<F = F>,
    OutputDrawable: Drawable<OpSetT>,
{
    // This trait contains all primitive shapes
    // type OutputDrawable: Drawable<OpSetT: OpSetTrait, F=F>;

    fn line(&self, x1: F, y1: F, x2: F, y2: F, options: &Option<DrawOptions>) -> OutputDrawable;

    fn rectangle(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn ellipse(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn circle(&self, x: F, y: F, diameter: F, options: &Option<DrawOptions>) -> OutputDrawable;

    fn linear_path(
        &self,
        points: &[Point2<F>],
        close: bool,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn polygon(&self, points: &[Point2<F>], options: &Option<DrawOptions>) -> OutputDrawable;

    fn arc(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        start: F,
        stop: F,
        closed: bool,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn bezier_quadratic(
        &self,
        start: Point2<F>,
        cp: Point2<F>,
        end: Point2<F>,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn bezier_cubic(
        &self,
        start: Point2<F>,
        cp1: Point2<F>,
        cp2: Point2<F>,
        end: Point2<F>,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn curve(&self, points: &[Point2<F>], options: &Option<DrawOptions>) -> OutputDrawable;

    fn path(&self, svg_path: String, options: &Option<DrawOptions>) -> OutputDrawable;
}
