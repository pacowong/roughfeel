use std::{fmt::Display, ops::MulAssign};

use nalgebra::{Point2, Scalar};
use nalgebra_glm::RealNumber;

use crate::graphics::{
    drawable::{DrawOptions, Drawable},
    drawable_maker::{Generator, RoughlyDrawableMakable},
    drawable_ops::OpSet,
};

use super::kurbo_drawable::{KurboDrawable, KurboOpSet, ToKurboDrawable};

use std::marker::PhantomData;

#[derive(Default)]
pub struct KurboDrawableMaker<F: RealNumber + Display, OutputDrawable: Drawable<KurboOpSet<F>>> {
    gen: Generator<OpSet<F>>,
    phantom_data_f: PhantomData<F>,
    phantom_data_output_drawable: PhantomData<OutputDrawable>,
}

impl<
        F: RealNumber + MulAssign + Display,
        OutputDrawable: Drawable<KurboOpSet<F>>,
    > KurboDrawableMaker<F, OutputDrawable>
{
    pub fn new(gen: Generator<OpSet<F>>) -> Self {
        Self {
            gen,
            phantom_data_f: PhantomData,
            phantom_data_output_drawable: PhantomData,
        }
    }
}

impl<F: RealNumber + MulAssign + Display, OutputDrawable: Drawable<KurboOpSet<F>>>
    RoughlyDrawableMakable<F, KurboOpSet<F>, KurboDrawable<F>>
    for KurboDrawableMaker<F, OutputDrawable>
{
    fn line(&self, x1: F, y1: F, x2: F, y2: F, options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.line(x1, y1, x2, y2, options);
        drawable.to_kurbo_drawable()
    }

    fn rectangle(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.rectangle(x, y, width, height, options);
        drawable.to_kurbo_drawable()
    }

    fn ellipse(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.ellipse(x, y, width, height, options);
        drawable.to_kurbo_drawable()
    }

    fn circle(&self, x: F, y: F, diameter: F, options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.circle(x, y, diameter, options);
        drawable.to_kurbo_drawable()
    }

    fn linear_path(
        &self,
        points: &[Point2<F>],
        close: bool,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.linear_path(points, close, options);
        drawable.to_kurbo_drawable()
    }

    fn polygon(&self, points: &[Point2<F>], options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.polygon(points, options);
        drawable.to_kurbo_drawable()
    }

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
    ) -> KurboDrawable<F> {
        let drawable = self
            .gen
            .arc(x, y, width, height, start, stop, closed, options);
        drawable.to_kurbo_drawable()
    }

    fn bezier_quadratic(
        &self,
        start: Point2<F>,
        cp: Point2<F>,
        end: Point2<F>,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.bezier_quadratic(start, cp, end, options);
        drawable.to_kurbo_drawable()
    }

    fn bezier_cubic(
        &self,
        start: Point2<F>,
        cp1: Point2<F>,
        cp2: Point2<F>,
        end: Point2<F>,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.bezier_cubic(start, cp1, cp2, end, options);
        drawable.to_kurbo_drawable()
    }

    fn curve(&self, points: &[Point2<F>], options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.curve(points, options);
        drawable.to_kurbo_drawable()
    }

    fn path(&self, svg_path: String, options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.path(svg_path, options);
        drawable.to_kurbo_drawable()
    }
}
