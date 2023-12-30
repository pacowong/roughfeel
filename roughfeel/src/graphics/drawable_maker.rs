use std::fmt::{Display, Write};
use std::marker::PhantomData;
use std::ops::MulAssign;

use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};
use points_on_curve::{curve_to_bezier, points_on_bezier_curves};

use crate::graphics::_c;
use crate::graphics::drawable::{DrawOptions, DrawOptionsBuilder, Drawable, PathInfo};
use crate::graphics::drawable_ops::{OpSet, OpSetType, OpType};
use crate::graphics::geometry::{convert_bezier_quadratic_to_cubic, BezierQuadratic};
use crate::graphics::paint::FillStyle;
use crate::graphics::points_on_path::points_on_path;
use crate::graphics::renderer::{
    bezier_cubic, bezier_quadratic, curve, ellipse_with_params, generate_ellipse_params, line,
    linear_path, pattern_fill_arc, pattern_fill_polygons, rectangle, solid_fill_polygon, svg_path,
};

use super::drawable::{OpSetTrait, RoughlyDrawable};
use super::render_context::RoughlyCanvas;

pub struct Generator<OpSetT: OpSetTrait> {
    default_options: DrawOptions,
    phantom_data_opsett: PhantomData<OpSetT>,
}

impl<F: Trig + Float, OpSetT: OpSetTrait<F = F>> Default for Generator<OpSetT> {
    fn default() -> Self {
        Self {
            default_options: DrawOptionsBuilder::default()
                .seed(345_u64)
                .build()
                .expect("failed to build default options"),
            phantom_data_opsett: PhantomData,
        }
    }
}

//impl<F: Trig + Float, OpSetT: OpSetTrait<F = F>> Generator<OpSetT> 
impl<F: Trig + Float> Generator<OpSet<F>>
{
    pub fn new(options: DrawOptions) -> Self {
        Generator {
            default_options: options,
            phantom_data_opsett: PhantomData,
        }
    }

    // fn d(&self, name: T, op_sets: &[OpSet<F>], options: &Option<DrawOptions>) -> RoughlyDrawable<F>
    // where
    //     T: Into<String>,
    fn d(
        &self,
        name: String,
        op_sets: &[OpSet<F>],
        options: &Option<DrawOptions>,
    ) -> RoughlyDrawable<OpSet<F>> {
        RoughlyDrawable::<OpSet<F>>::draw(
            name.into(),
            options
                .clone()
                .unwrap_or_else(|| self.default_options.clone()),
            Vec::from_iter(op_sets.iter().cloned()),
        )
    }

    pub fn ops_to_path(mut drawing: OpSet<F>, fixed_decimals: Option<u32>) -> String
    where
        F: Float + FromPrimitive + Trig + Display,
    {
        let mut path = String::new();

        for item in drawing.ops.iter_mut() {
            if let Some(fd) = fixed_decimals {
                let pow: u32 = 10u32.pow(fd);
                item.data.iter_mut().for_each(|p| {
                    *p = (*p * F::from(pow).unwrap()).round() / F::from(pow).unwrap();
                });
            }

            match item.op {
                OpType::Move => {
                    write!(&mut path, "L{} {} ", item.data[0], item.data[1])
                        .expect("Failed to write path string");
                }
                OpType::BCurveTo => {
                    write!(
                        &mut path,
                        "C{} {}, {} {}, {} {} ",
                        item.data[0],
                        item.data[1],
                        item.data[2],
                        item.data[3],
                        item.data[4],
                        item.data[5]
                    )
                    .expect("Failed to write path string");
                }
                OpType::LineTo => {
                    write!(&mut path, "L{} {}, ", item.data[0], item.data[1])
                        .expect("Failed to write path string");
                }
            }
        }

        path
    }

    pub fn to_paths(drawable: RoughlyDrawable<OpSet<F>>) -> Vec<PathInfo>
    where
        F: Float + FromPrimitive + Trig + Display,
    {
        let sets = drawable.opsets;
        let o = drawable.options;
        let mut path_infos = vec![];
        for drawing in sets.iter() {
            let path_info = match drawing.op_set_type {
                OpSetType::Path => PathInfo {
                    d: Self::ops_to_path(drawing.clone(), None),
                    stroke: o.stroke,
                    stroke_width: o.stroke_width,
                    fill: None,
                },
                OpSetType::FillPath => PathInfo {
                    d: Self::ops_to_path(drawing.clone(), None),
                    stroke: None,
                    stroke_width: Some(0.0f32),
                    fill: o.fill,
                },
                OpSetType::FillSketch => {
                    let fill_weight = if o.fill_weight.unwrap_or(0.0) < 0.0 {
                        o.stroke_width.unwrap_or(0.0) / 2.0
                    } else {
                        o.fill_weight.unwrap_or(0.0)
                    };
                    PathInfo {
                        d: Self::ops_to_path(drawing.clone(), None),
                        stroke: o.fill,
                        stroke_width: Some(fill_weight),
                        fill: None,
                    }
                }
            };
            path_infos.push(path_info);
        }
        path_infos
    }
}

pub trait RoughlyDrawableMaker<
    F: Trig + Float + FromPrimitive + MulAssign + Display,
    OpSetT,
    OutputDrawable,
> where
    //OutputDrawable: Drawable<OpSet<F> >, //Not OK, only one kind of opset
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
        points: &[Point2D<F>],
        close: bool,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn polygon(&self, points: &[Point2D<F>], options: &Option<DrawOptions>) -> OutputDrawable;

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
        start: Point2D<F>,
        cp: Point2D<F>,
        end: Point2D<F>,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn bezier_cubic(
        &self,
        start: Point2D<F>,
        cp1: Point2D<F>,
        cp2: Point2D<F>,
        end: Point2D<F>,
        options: &Option<DrawOptions>,
    ) -> OutputDrawable;

    fn curve(&self, points: &[Point2D<F>], options: &Option<DrawOptions>) -> OutputDrawable;

    fn path(&self, svg_path: String, options: &Option<DrawOptions>) -> OutputDrawable;
}

//impl<T, F: Trig + Float + FromPrimitive + MulAssign + Display> RoughlyDrawableMaker<RoughlyDrawable<F>, F> for Generator<T, F, RoughlyDrawable<F> > { //Work
//impl<T, F: Trig + Float + FromPrimitive + MulAssign + Display, OpSetT: OpSetTrait<F = F>, OutputDrawable: Drawable<OpSetT, F = F> > RoughlyDrawableMaker<RoughlyDrawable<F>, OutputDrawable > for Generator<T, F, OutputDrawable > {
impl<F: Trig + Float + FromPrimitive + MulAssign + Display>
    RoughlyDrawableMaker<F, OpSet<F>, RoughlyDrawable<OpSet<F>>> for Generator<OpSet<F>>
// <OutputDrawable as Drawable>::F: F
{
    // fn d<T, F>(&self, name: T, op_sets: &[OpSet<F>], options: &Option<DrawOptions>) -> RoughlyDrawable<F>
    // where
    //     T: Into<String>,
    //     F: Float + Trig + FromPrimitive,
    // {
    //     RoughlyDrawable {
    //         shape: name.into(),
    //         options: options
    //             .clone()
    //             .unwrap_or_else(|| self.default_options.clone()),
    //         sets: Vec::from_iter(op_sets.iter().cloned()),
    //     }
    // }

    fn line(&self, x1: F, y1: F, x2: F, y2: F, options: &Option<DrawOptions>) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive,
    {
        let x = self.d(
            "line".to_owned(),
            &[line(
                x1,
                y1,
                x2,
                y2,
                &mut options
                    .clone()
                    .unwrap_or_else(|| self.default_options.clone()),
            )],
            options,
        );
        x
    }

    fn rectangle(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: &Option<DrawOptions>,
    ) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive,
    {
        let mut paths = vec![];
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());
        let outline = rectangle(x, y, width, height, &mut options);
        if options.fill.is_some() {
            let points = vec![
                Point2D::new(x, y),
                Point2D::new(x + width, y),
                Point2D::new(x + width, y + height),
                Point2D::new(x, y + height),
            ];
            if options.fill_style == Some(FillStyle::Solid) {
                paths.push(solid_fill_polygon(&vec![points], &mut options));
            } else {
                paths.push(pattern_fill_polygons(vec![points], &mut options));
            }
        }
        if options.stroke.is_some() {
            paths.push(outline);
        }

        self.d("rectangle".to_owned(), &paths, &Some(options))
    }

    fn ellipse(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        options: &Option<DrawOptions>,
    ) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive,
    {
        let mut paths = vec![];
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());
        let ellipse_params = generate_ellipse_params(width, height, &mut options);
        let ellipse_response = ellipse_with_params(x, y, &mut options, &ellipse_params);
        if options.fill.is_some() {
            if options.fill_style == Some(FillStyle::Solid) {
                let mut shape = ellipse_with_params(x, y, &mut options, &ellipse_params).opset;
                shape.op_set_type = OpSetType::FillPath;
                paths.push(shape);
            } else {
                paths.push(pattern_fill_polygons(
                    vec![ellipse_response.estimated_points],
                    &mut options,
                ));
            }
        }
        if options.stroke.is_some() {
            paths.push(ellipse_response.opset);
        }
        self.d("ellipse".to_owned(), &paths, &Some(options))
    }

    fn circle(&self, x: F, y: F, diameter: F, options: &Option<DrawOptions>) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive,
    {
        let mut shape = self.ellipse(x, y, diameter, diameter, options);
        shape.shape = "circle".into();
        shape
    }

    fn linear_path(
        &self,
        points: &[Point2D<F>],
        close: bool,
        options: &Option<DrawOptions>,
    ) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive,
    {
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());
        self.d(
            "linear_path".to_owned(),
            &[linear_path(points, close, &mut options)],
            &Some(options),
        )
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
    ) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive,
    {
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());
        let mut paths = vec![];
        let outline = crate::graphics::renderer::arc(
            x,
            y,
            width,
            height,
            start,
            stop,
            closed,
            true,
            &mut options,
        );
        if closed && options.fill.is_some() {
            if options.fill_style == Some(FillStyle::Solid) {
                options.disable_multi_stroke = Some(true);
                let mut shape = crate::graphics::renderer::arc(
                    x,
                    y,
                    width,
                    height,
                    start,
                    stop,
                    true,
                    false,
                    &mut options,
                );
                shape.op_set_type = OpSetType::FillPath;
                paths.push(shape);
            } else {
                paths.push(pattern_fill_arc(
                    x,
                    y,
                    width,
                    height,
                    start,
                    stop,
                    &mut options,
                ));
            }
        }
        if options.stroke.is_some() {
            paths.push(outline);
        }
        self.d("arc".to_owned(), &paths, &Some(options))
    }

    fn bezier_quadratic(
        &self,
        start: Point2D<F>,
        cp: Point2D<F>,
        end: Point2D<F>,
        options: &Option<DrawOptions>,
    ) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive + MulAssign + Display,
    {
        let mut paths = vec![];
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());

        let outline = bezier_quadratic(start, cp, end, &mut options);

        if options.fill.is_some() {
            // The fill algorithms expect at least 4 points of a cubic curve, else they panic
            let cubic = convert_bezier_quadratic_to_cubic(BezierQuadratic { start, cp, end });
            let crv = vec![cubic.start, cubic.cp1, cubic.cp2, cubic.end];

            let poly_points = points_on_bezier_curves(
                &crv,
                _c(10.0),
                Some(_c::<F>(1.0) + _c::<F>(options.roughness.unwrap_or(0.0)) / _c(2.0)),
            );
            if options.fill_style == Some(FillStyle::Solid) {
                paths.push(solid_fill_polygon(&vec![poly_points], &mut options));
            } else {
                paths.push(pattern_fill_polygons(&mut vec![poly_points], &mut options));
            }
        }

        if options.stroke.is_some() {
            paths.push(outline);
        }

        self.d("curve".to_owned(), &paths, &Some(options))
    }

    fn bezier_cubic(
        &self,
        start: Point2D<F>,
        cp1: Point2D<F>,
        cp2: Point2D<F>,
        end: Point2D<F>,
        options: &Option<DrawOptions>,
    ) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive + MulAssign + Display,
    {
        let mut paths = vec![];
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());

        let outline = bezier_cubic(start, cp1, cp2, end, &mut options);

        if options.fill.is_some() {
            let crv = vec![start, cp1, cp2, end];

            let poly_points = points_on_bezier_curves(
                &crv,
                _c(10.0),
                Some(_c::<F>(1.0) + _c::<F>(options.roughness.unwrap_or(0.0)) / _c(2.0)),
            );
            if options.fill_style == Some(FillStyle::Solid) {
                paths.push(solid_fill_polygon(&vec![poly_points], &mut options));
            } else {
                paths.push(pattern_fill_polygons(&mut vec![poly_points], &mut options));
            }
        }

        if options.stroke.is_some() {
            paths.push(outline);
        }

        self.d("curve".to_owned(), &paths, &Some(options))
    }

    fn curve(&self, points: &[Point2D<F>], options: &Option<DrawOptions>) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive + MulAssign + Display,
    {
        let mut paths = vec![];
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());
        let outline = curve(points, &mut options);
        if options.fill.is_some() && points.len() >= 3 {
            let curve = curve_to_bezier(points, _c(0.0));
            if let Some(crv) = curve {
                let poly_points = points_on_bezier_curves(
                    &crv,
                    _c(10.0),
                    Some(_c::<F>(1.0) + _c::<F>(options.roughness.unwrap_or(0.0)) / _c(2.0)),
                );
                if options.fill_style == Some(FillStyle::Solid) {
                    paths.push(solid_fill_polygon(&vec![poly_points], &mut options));
                } else {
                    paths.push(pattern_fill_polygons(&mut vec![poly_points], &mut options));
                }
            }
        }

        if options.stroke.is_some() {
            paths.push(outline);
        }

        self.d("curve".to_owned(), &paths, &Some(options))
    }

    fn polygon(&self, points: &[Point2D<F>], options: &Option<DrawOptions>) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive + MulAssign + Display,
    {
        let mut options = options
            .clone()
            .unwrap_or_else(|| self.default_options.clone());
        let mut paths = vec![];
        let outline = linear_path(points, true, &mut options);
        if options.fill.is_some() {
            if options.fill_style == Some(FillStyle::Solid) {
                paths.push(solid_fill_polygon(&vec![points.to_vec()], &mut options));
            } else {
                paths.push(pattern_fill_polygons(
                    &mut vec![points.to_vec()],
                    &mut options,
                ));
            }
        }
        if options.stroke.is_some() {
            paths.push(outline);
        }
        self.d("polygon".to_owned(), &paths, &Some(options))
    }

    fn path(&self, d: String, options: &Option<DrawOptions>) -> RoughlyDrawable<OpSet<F>>
    where
        F: Float + Trig + FromPrimitive + MulAssign + Display,
    {
        let mut options = options.clone().unwrap_or(self.default_options.clone());
        let mut paths = vec![];
        if d.is_empty() {
            self.d("path".to_owned(), &paths, &Some(options))
        } else {
            let simplified = options.simplification.map(|a| a < 1.0).unwrap_or(false);
            let distance = if simplified {
                _c::<F>(4.0) - _c::<F>(4.0) * _c::<F>(options.simplification.unwrap())
            } else {
                (_c::<F>(1.0) + _c::<F>(options.roughness.unwrap_or(1.0))) / _c::<F>(2.0)
            };

            let sets = points_on_path(d.clone(), Some(_c(1.0)), Some(distance));
            if options.fill.is_some() {
                if options.fill_style == Some(FillStyle::Solid) {
                    paths.push(solid_fill_polygon(&sets, &mut options));
                } else {
                    paths.push(pattern_fill_polygons(sets.clone(), &mut options));
                }
            }

            if options.stroke.is_some() {
                if simplified {
                    sets.iter()
                        .for_each(|s| paths.push(linear_path(s, false, &mut options)));
                } else {
                    paths.push(svg_path(d, &mut options));
                }
            }

            self.d("path".to_owned(), &paths, &Some(options))
        }
    }
}
