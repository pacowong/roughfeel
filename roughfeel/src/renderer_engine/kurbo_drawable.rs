use std::fmt::Display;
use std::ops::MulAssign;

use crate::graphics::_to_f64;
use crate::graphics::drawable::{DrawOptions, DrawOptionsBuilder, OpSetTrait, RoughlyDrawable};
use palette::rgb::Rgba;
use palette::Srgba;
use piet::kurbo::{BezPath, PathEl, Point};
use piet::{Color, LineJoin, RenderContext, StrokeStyle};

use nalgebra::{Point2, Scalar};
use nalgebra_glm::RealNumber;

use crate::graphics::drawable_ops::{OpSet, OpSetType, OpType};

use crate::graphics::drawable::Drawable;

#[derive(Clone)]
pub struct KurboOpSet<F: RealNumber> {
    pub op_set_type: OpSetType,
    pub ops: BezPath,
    pub size: Option<Point2<F>>,
    pub path: Option<String>,
}

impl<F: RealNumber> OpSetTrait for KurboOpSet<F> {
    type F = F;
}

pub struct KurboDrawable<F: RealNumber> {
    pub shape: String,
    pub options: DrawOptions,
    pub sets: Vec<KurboOpSet<F>>,
}

// impl Default for KurboDrawable {
//     fn default() -> Self {
//         Self {
//             default_options: DrawOptionsBuilder::default()
//                 .seed(345_u64)
//                 .build()
//                 .expect("failed to build default options"),
//         }
//     }
// }

impl<FT: RealNumber> Drawable<KurboOpSet<FT>> for KurboDrawable<FT> {
    // type F = FT;

    // fn draw(
    //     shape: String,
    //     options: DrawOptions,
    //     sets: Vec<OpSet<Self::F>>) -> KurboDrawable<FT> {

    // }

    fn draw(shape: String, options: DrawOptions, sets: Vec<KurboOpSet<FT>>) -> KurboDrawable<FT> {
        Self {
            shape: shape.into(),
            options: options.clone(),
            // .unwrap_or_else(|| self.default_options.clone()),
            sets: Vec::from_iter(sets.iter().cloned()),
        }
    }
}

impl<F: RealNumber> KurboDrawable<F> {
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
                        ss.set_line_cap(convert_line_cap_from_roughr_to_piet(
                            self.options.line_cap,
                        ));
                        ss.set_line_join(convert_line_join_from_roughr_to_piet(
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
                        ss.set_line_cap(convert_line_cap_from_roughr_to_piet(
                            self.options.line_cap,
                        ));
                        ss.set_line_join(convert_line_join_from_roughr_to_piet(
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

pub trait ToKurboOpset<F: RealNumber> {
    fn to_kurbo_opset(self) -> KurboOpSet<F>;
}

impl<F: RealNumber> ToKurboOpset<F> for OpSet<F> {
    fn to_kurbo_opset(self) -> KurboOpSet<F> {
        KurboOpSet {
            op_set_type: self.op_set_type.clone(),
            size: self.size,
            path: self.path.clone(),
            ops: opset_to_shape(&self),
        }
    }
}

fn opset_to_shape<F: RealNumber>(op_set: &OpSet<F>) -> BezPath {
    let mut path: BezPath = BezPath::new();
    for item in op_set.ops.iter() {
        match item.op {
            OpType::Move => path.extend([PathEl::MoveTo(Point::new(
                _to_f64(item.data[0]),
                _to_f64(item.data[1]),
            ))]),
            OpType::BCurveTo => path.extend([PathEl::CurveTo(
                Point::new(_to_f64(item.data[0]), _to_f64(item.data[1])),
                Point::new(_to_f64(item.data[2]), _to_f64(item.data[3])),
                Point::new(_to_f64(item.data[4]), _to_f64(item.data[5])),
            )]),
            OpType::LineTo => {
                path.extend([PathEl::LineTo(Point::new(
                    _to_f64(item.data[0]),
                    _to_f64(item.data[1]),
                ))]);
            }
        }
    }
    path
}

pub trait ToKurboDrawable<F: RealNumber> {
    fn to_kurbo_drawable(self) -> KurboDrawable<F>;
}

impl<F: RealNumber> ToKurboDrawable<F> for RoughlyDrawable<OpSet<F>> {
    fn to_kurbo_drawable(self) -> KurboDrawable<F> {
        KurboDrawable {
            shape: self.shape,
            options: self.options,
            sets: self
                .opsets
                .into_iter()
                .map(|s| s.to_kurbo_opset())
                .collect(),
        }
    }
}

fn convert_line_cap_from_roughr_to_piet(
    roughr_line_cap: Option<crate::graphics::paint::LineCap>,
) -> piet::LineCap {
    match roughr_line_cap {
        Some(crate::graphics::paint::LineCap::Butt) => piet::LineCap::Butt,
        Some(crate::graphics::paint::LineCap::Round) => piet::LineCap::Round,
        Some(crate::graphics::paint::LineCap::Square) => piet::LineCap::Square,
        None => piet::LineCap::Butt,
    }
}

fn convert_line_join_from_roughr_to_piet(
    roughr_line_join: Option<crate::graphics::paint::LineJoin>,
) -> LineJoin {
    match roughr_line_join {
        Some(crate::graphics::paint::LineJoin::Miter { limit }) => LineJoin::Miter { limit },
        Some(crate::graphics::paint::LineJoin::Round) => LineJoin::Round,
        Some(crate::graphics::paint::LineJoin::Bevel) => LineJoin::Bevel,
        None => LineJoin::Miter {
            limit: LineJoin::DEFAULT_MITER_LIMIT,
        },
    }
}
