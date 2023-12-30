use std::borrow::BorrowMut;
use std::marker::PhantomData;

use euclid::default::Point2D;
use euclid::{point2, Trig};
use num_traits::{Float, FloatConst, FromPrimitive};

use super::scan_line_hachure::polygon_hachure_lines;

use super::traits::PatternFiller;
use crate::graphics::_c;
use crate::graphics::drawable::DrawOptions;
use crate::graphics::drawable_ops::{Op, OpSet, OpSetType};
use crate::graphics::geometry::{rotate_lines, rotate_points, Line};

pub struct ZigZagFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for ZigZagFiller<F>
where
    F: Float + Trig + FromPrimitive,
    P: BorrowMut<Vec<Vec<Point2D<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut DrawOptions) -> OpSet<F> {
        let mut gap = o.hachure_gap.map(_c::<F>).unwrap_or_else(|| _c::<F>(-1.0));
        if gap < F::zero() {
            gap = o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c::<F>(1.0)) * _c::<F>(4.0);
        }
        gap = gap.max(_c::<F>(0.1));
        let mut o2 = o.clone();
        o2.set_hachure_gap(Some(gap.to_f32().unwrap()));
        let lines = polygon_hachure_lines(polygon_list.borrow_mut(), &o2);
        let zig_zag_angle =
            (_c::<F>(f32::PI()) / _c::<F>(180.0)) * _c::<F>(o.hachure_angle.unwrap_or(0.0));
        let mut zig_zag_lines = vec![];
        let dgx = gap * _c::<F>(0.5) * Trig::cos(zig_zag_angle);
        let dgy = gap * _c::<F>(0.5) * Trig::sin(zig_zag_angle);

        for line in lines.iter() {
            if line.length() > _c::<F>(0.0) {
                zig_zag_lines.push(Line {
                    start_point: point2(line.start_point.x - dgx, line.start_point.y + dgy),
                    end_point: line.end_point,
                });
                zig_zag_lines.push(Line {
                    start_point: point2(line.start_point.x + dgx, line.start_point.y - dgy),
                    end_point: line.end_point,
                });
            }
        }

        let ops = ZigZagFiller::render_lines(zig_zag_lines, o);
        return OpSet {
            ops,
            op_set_type: OpSetType::FillSketch,
            size: None,
            path: None,
        };
    }
}

impl<F: Float + Trig + FromPrimitive> ZigZagFiller<F> {
    pub fn new() -> Self {
        ZigZagFiller {
            _phantom: PhantomData,
        }
    }

    fn render_lines(lines: Vec<Line<F>>, o: &mut DrawOptions) -> Vec<Op<F>> {
        let mut ops: Vec<Op<F>> = vec![];
        lines.iter().for_each(|l| {
            ops.extend(crate::graphics::renderer::_double_line(
                l.start_point.x,
                l.start_point.y,
                l.end_point.x,
                l.end_point.y,
                o,
                true,
            ))
        });

        ops
    }
}
