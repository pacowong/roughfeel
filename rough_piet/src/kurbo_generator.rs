use std::fmt::Display;
use std::ops::MulAssign;

use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};
use palette::rgb::Rgba;
use palette::Srgba;
use piet::kurbo::{BezPath, PathEl, Point};
use piet::{Color, LineJoin, RenderContext, StrokeStyle};
use roughfeel::graphics::{drawable::RoughlyDrawable, drawable_ops::OpSet, drawable_ops::OpSetType, drawable_ops::OpType, drawable::DrawOptions};
use roughfeel::graphics::drawable_maker::Generator;

#[derive(Default)]
pub struct KurboGenerator {
    gen: Generator::<OpSet<f32>>,
    options: Option<DrawOptions>,
}

#[derive(Clone)]
pub struct KurboOpset<F: Float + Trig> {
    pub op_set_type: OpSetType,
    pub ops: BezPath,
    pub size: Option<Point2D<F>>,
    pub path: Option<String>,
}

pub trait ToKurboOpset<F: Float + Trig> {
    fn to_kurbo_opset(self) -> KurboOpset<F>;
}

impl<F: Float + Trig + FromPrimitive> ToKurboOpset<F> for OpSet<F> {
    fn to_kurbo_opset(self) -> KurboOpset<F> {
        KurboOpset {
            op_set_type: self.op_set_type.clone(),
            size: self.size,
            path: self.path.clone(),
            ops: opset_to_shape(&self),
        }
    }
}

pub struct KurboDrawable<F: Float + Trig> {
    pub shape: String,
    pub options: DrawOptions,
    pub sets: Vec<KurboOpset<F>>,
}

pub trait ToKurboDrawable<F: Float + Trig> {
    fn to_kurbo_drawable(self) -> KurboDrawable<F>;
}

impl<F: Float + Trig + FromPrimitive> ToKurboDrawable<F> for RoughlyDrawable<F> {
    fn to_kurbo_drawable(self) -> KurboDrawable<F> {
        KurboDrawable {
            shape: self.shape,
            options: self.options,
            sets: self.sets.into_iter().map(|s| s.to_kurbo_opset()).collect(),
        }
    }
}

impl KurboGenerator {
    pub fn new(options: DrawOptions) -> Self {
        KurboGenerator { gen: Generator::default(), options: Some(options) }
    }
}

impl<F: Float + Trig> KurboDrawable<F> {
    pub fn draw(&self, ctx: &mut impl RenderContext) {
        for set in self.sets.iter() {
            match set.op_set_type {
                OpSetType::Path => {
                    ctx.save().expect("Failed to save render context");
                    if self.options.stroke_line_dash.is_some() {
                        let stroke_line_dash =
                            self.options.stroke_line_dash.clone().unwrap_or(Vec::new());
                        let mut ss = StrokeStyle::new();
                        ss.set_dash_pattern(stroke_line_dash.as_slice());
                        ss.set_dash_offset(self.options.stroke_line_dash_offset.unwrap_or(1.0f64));
                        ss.set_line_cap(convert_line_cap_from_roughfeel_to_piet(
                            self.options.line_cap,
                        ));
                        ss.set_line_join(convert_line_join_from_roughfeel_to_piet(
                            self.options.line_join,
                        ));

                        let stroke_color = self
                            .options
                            .stroke
                            .unwrap_or_else(|| Srgba::from_components((1.0, 1.0, 1.0, 1.0)));
                        let rgb: (f32, f32, f32, f32) = stroke_color.into_components();
                        ctx.stroke_styled(
                            set.ops.clone(),
                            &Color::rgba(rgb.0 as f64, rgb.1 as f64, rgb.2 as f64, rgb.3 as f64),
                            self.options.stroke_width.unwrap_or(1.0) as f64,
                            &ss,
                        );
                        ctx.restore().expect("Failed to restore render context");
                    } else {
                        let stroke_color = self
                            .options
                            .stroke
                            .unwrap_or_else(|| Srgba::new(1.0, 1.0, 1.0, 1.0));
                        let rgb: (f32, f32, f32, f32) = stroke_color.into_components();
                        ctx.stroke(
                            set.ops.clone(),
                            &Color::rgba(rgb.0 as f64, rgb.1 as f64, rgb.2 as f64, rgb.3 as f64),
                            self.options.stroke_width.unwrap_or(1.0) as f64,
                        );
                        ctx.restore().expect("Failed to restore render context");
                    }
                }
                OpSetType::FillPath => {
                    ctx.save().expect("Failed to save render context");
                    match self.shape.as_str() {
                        "curve" | "polygon" | "path" => {
                            let fill_color =
                                self.options.fill.unwrap_or(Rgba::new(1.0, 1.0, 1.0, 1.0));
                            let rgb: (f32, f32, f32, f32) = fill_color.into_components();
                            ctx.fill_even_odd(
                                set.ops.clone(),
                                &Color::rgba(
                                    rgb.0 as f64,
                                    rgb.1 as f64,
                                    rgb.2 as f64,
                                    rgb.3 as f64,
                                ),
                            )
                        }
                        _ => {
                            let fill_color =
                                self.options.fill.unwrap_or(Rgba::new(1.0, 1.0, 1.0, 1.0));
                            let rgb: (f32, f32, f32, f32) = fill_color.into_components();
                            ctx.fill(
                                set.ops.clone(),
                                &Color::rgba(
                                    rgb.0 as f64,
                                    rgb.1 as f64,
                                    rgb.2 as f64,
                                    rgb.3 as f64,
                                ),
                            )
                        }
                    }
                    ctx.restore().expect("Failed to restore render context");
                }
                OpSetType::FillSketch => {
                    let mut fweight = self.options.fill_weight.unwrap_or_default();
                    if fweight < 0.0 {
                        fweight = self.options.stroke_width.unwrap_or(1.0) / 2.0;
                    }
                    ctx.save().expect("Failed to save render context");

                    if self.options.fill_line_dash.is_some() {
                        let fill_line_dash =
                            self.options.fill_line_dash.clone().unwrap_or_default();
                        let mut ss = StrokeStyle::new();
                        ss.set_dash_pattern(fill_line_dash.as_slice());
                        ss.set_dash_offset(self.options.fill_line_dash_offset.unwrap_or(0.0f64));
                        ss.set_line_cap(convert_line_cap_from_roughfeel_to_piet(
                            self.options.line_cap,
                        ));
                        ss.set_line_join(convert_line_join_from_roughfeel_to_piet(
                            self.options.line_join,
                        ));
                        let fill_color = self
                            .options
                            .fill
                            .unwrap_or_else(|| Rgba::new(1.0, 1.0, 1.0, 1.0));
                        let rgb: (f32, f32, f32, f32) = fill_color.into_components();
                        ctx.stroke_styled(
                            set.ops.clone(),
                            &Color::rgba(rgb.0 as f64, rgb.1 as f64, rgb.2 as f64, rgb.3 as f64),
                            fweight as f64,
                            &ss,
                        );
                    } else {
                        let fill_color = self
                            .options
                            .fill
                            .unwrap_or_else(|| Rgba::new(1.0, 1.0, 1.0, 1.0));
                        let rgb: (f32, f32, f32, f32) = fill_color.into_components();
                        ctx.stroke(
                            set.ops.clone(),
                            &Color::rgba(rgb.0 as f64, rgb.1 as f64, rgb.2 as f64, rgb.3 as f64),
                            fweight as f64,
                        );
                    }
                    ctx.restore().expect("Failed to restore render context");
                }
            }
        }
    }
}

fn opset_to_shape<F: Trig + Float + FromPrimitive>(op_set: &OpSet<F>) -> BezPath {
    let mut path: BezPath = BezPath::new();
    for item in op_set.ops.iter() {
        match item.op {
            OpType::Move => path.extend([PathEl::MoveTo(Point::new(
                item.data[0].to_f64().unwrap(),
                item.data[1].to_f64().unwrap(),
            ))]),
            OpType::BCurveTo => path.extend([PathEl::CurveTo(
                Point::new(
                    item.data[0].to_f64().unwrap(),
                    item.data[1].to_f64().unwrap(),
                ),
                Point::new(
                    item.data[2].to_f64().unwrap(),
                    item.data[3].to_f64().unwrap(),
                ),
                Point::new(
                    item.data[4].to_f64().unwrap(),
                    item.data[5].to_f64().unwrap(),
                ),
            )]),
            OpType::LineTo => {
                path.extend([PathEl::LineTo(Point::new(
                    item.data[0].to_f64().unwrap(),
                    item.data[1].to_f64().unwrap(),
                ))]);
            }
        }
    }
    path
}

impl KurboGenerator {
    pub fn line<F: Trig + Float + FromPrimitive>(
        &self,
        x1: F,
        y1: F,
        x2: F,
        y2: F,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.line(x1, y1, x2, y2, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn rectangle<F: Trig + Float + FromPrimitive>(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.rectangle(x, y, width, height, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn ellipse<F: Trig + Float + FromPrimitive>(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.ellipse(x, y, width, height, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn circle<F: Trig + Float + FromPrimitive>(
        &self,
        x: F,
        y: F,
        diameter: F,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.circle(x, y, diameter, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn linear_path<F: Trig + Float + FromPrimitive>(
        &self,
        points: &[Point2D<F>],
        close: bool,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.linear_path(points, close, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn polygon<F: Trig + Float + FromPrimitive + MulAssign + Display>(
        &self,
        points: &[Point2D<F>],
    ) -> KurboDrawable<F> {
        let drawable = self.gen.polygon(points, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn arc<F: Trig + Float + FromPrimitive>(
        &self,
        x: F,
        y: F,
        width: F,
        height: F,
        start: F,
        stop: F,
        closed: bool,
    ) -> KurboDrawable<F> {
        let drawable = self
            .gen
            .arc(x, y, width, height, start, stop, closed, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn bezier_quadratic<F: Trig + Float + FromPrimitive + MulAssign + Display>(
        &self,
        start: Point2D<F>,
        cp: Point2D<F>,
        end: Point2D<F>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.bezier_quadratic(start, cp, end, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn bezier_cubic<F: Trig + Float + FromPrimitive + MulAssign + Display>(
        &self,
        start: Point2D<F>,
        cp1: Point2D<F>,
        cp2: Point2D<F>,
        end: Point2D<F>,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.bezier_cubic(start, cp1, cp2, end, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn curve<F: Trig + Float + FromPrimitive + MulAssign + Display>(
        &self,
        points: &[Point2D<F>],
    ) -> KurboDrawable<F> {
        let drawable = self.gen.curve(points, &self.options);
        drawable.to_kurbo_drawable()
    }

    pub fn path<F: Trig + Float + FromPrimitive + MulAssign + Display>(
        &self,
        svg_path: String,
    ) -> KurboDrawable<F> {
        let drawable = self.gen.path(svg_path, &self.options);
        drawable.to_kurbo_drawable()
    }
}

fn convert_line_cap_from_roughfeel_to_piet(
    roughfeel_line_cap: Option<roughfeel::graphics::paint::LineCap>,
) -> piet::LineCap {
    match roughfeel_line_cap {
        Some(roughfeel::graphics::paint::LineCap::Butt) => piet::LineCap::Butt,
        Some(roughfeel::graphics::paint::LineCap::Round) => piet::LineCap::Round,
        Some(roughfeel::graphics::paint::LineCap::Square) => piet::LineCap::Square,
        None => piet::LineCap::Butt,
    }
}

fn convert_line_join_from_roughfeel_to_piet(
    roughfeel_line_join: Option<roughfeel::graphics::paint::LineJoin>,
) -> LineJoin {
    match roughfeel_line_join {
        Some(roughfeel::graphics::paint::LineJoin::Miter { limit }) => LineJoin::Miter { limit },
        Some(roughfeel::graphics::paint::LineJoin::Round) => LineJoin::Round,
        Some(roughfeel::graphics::paint::LineJoin::Bevel) => LineJoin::Bevel,
        None => LineJoin::Miter { limit: LineJoin::DEFAULT_MITER_LIMIT },
    }
}
