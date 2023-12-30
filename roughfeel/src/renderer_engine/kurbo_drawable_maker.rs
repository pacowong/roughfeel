use std::{fmt::Display, ops::MulAssign};

use num_traits::{Float, FromPrimitive};

use euclid::{default::Point2D, Trig};

use crate::graphics::{
    drawable::{DrawOptions, Drawable, RoughlyDrawable},
    drawable_maker::{Generator, RoughlyDrawableMaker},
    drawable_ops::OpSet,
};

use super::kurbo_drawable::{KurboDrawable, KurboOpSet, ToKurboDrawable};

use std::marker::PhantomData;

#[derive(Default)]
pub struct KurboDrawableMaker<
    F: Trig + Float + FromPrimitive + MulAssign + Display,
    OutputDrawable: Drawable<KurboOpSet<F>>,
> {
    //gen: Generator<T, F, OutputDrawable>, //RoughlyDrawableMaker<RoughlyDrawable<F>, F> >,
    gen: Generator<F, OpSet<F>>,
    options: Option<DrawOptions>,
    phantom_data_f: PhantomData<F>,
    phantom_data_output_drawable: PhantomData<OutputDrawable>,
}

impl<F: Float + Trig + FromPrimitive + MulAssign + Display,
        OutputDrawable: Drawable<KurboOpSet<F>>,
    > KurboDrawableMaker<F, OutputDrawable>
{
    pub fn new(gen: Generator<F, OpSet<F>>, options: Option<DrawOptions>) -> Self {
        Self {
            gen,
            options,
            phantom_data_f: PhantomData,
            phantom_data_output_drawable: PhantomData,
        }
    }
}

// impl<T, F: Trig + Float + FromPrimitive + MulAssign + Display> RoughlyDrawableMaker<RoughlyDrawable<F>, F> for Generator<T, F, RoughlyDrawable<F> >
//impl<T, F: Trig + Float + FromPrimitive + MulAssign + Display, OutputDrawable: Drawable> RoughlyDrawableMaker<RoughlyDrawable<F>, F> for Generator<T, F, OutputDrawable>
impl<
        F: Float + Trig + FromPrimitive + MulAssign + Display,
        OutputDrawable: Drawable<KurboOpSet<F>>,
    > RoughlyDrawableMaker<F, KurboOpSet<F>, KurboDrawable<F>>
    for KurboDrawableMaker<F, OutputDrawable>
{
    fn line(&self, x1: F, y1: F, x2: F, y2: F, options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.line(x1, y1, x2, y2, &self.options);
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
        let drawable = self.gen.rectangle(x, y, width, height, &self.options);
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
        let drawable = self.gen.ellipse(x, y, width, height, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn circle(&self, x: F, y: F, diameter: F, options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.circle(x, y, diameter, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn linear_path(
        &self,
        points: &[Point2D<F>],
        close: bool,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.linear_path(points, close, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn polygon(&self, points: &[Point2D<F>], options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.polygon(points, &self.options);
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
            .arc(x, y, width, height, start, stop, closed, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn bezier_quadratic(
        &self,
        start: Point2D<F>,
        cp: Point2D<F>,
        end: Point2D<F>,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.bezier_quadratic(start, cp, end, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn bezier_cubic(
        &self,
        start: Point2D<F>,
        cp1: Point2D<F>,
        cp2: Point2D<F>,
        end: Point2D<F>,
        options: &Option<DrawOptions>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.bezier_cubic(start, cp1, cp2, end, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn curve(&self, points: &[Point2D<F>], options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.curve(points, &self.options);
        drawable.to_kurbo_drawable()
    }

    fn path(&self, svg_path: String, options: &Option<DrawOptions>) -> KurboDrawable<F> {
        let drawable = self.gen.path(svg_path, &self.options);
        drawable.to_kurbo_drawable()
    }
}
