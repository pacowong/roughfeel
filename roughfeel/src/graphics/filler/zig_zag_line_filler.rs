use std::borrow::BorrowMut;
use std::marker::PhantomData;

use nalgebra::Point2;
use nalgebra_glm::RealNumber;

use super::scan_line_hachure::polygon_hachure_lines;
use super::traits::PatternFiller;

use crate::graphics::drawable::DrawOptions;
use crate::graphics::drawable_ops::{Op, OpSet, OpSetType};
use crate::graphics::geometry::{rotate_lines, rotate_points, Line};
use crate::graphics::{_c, _to_f32, _to_u64, get_pi};

use crate::graphics::renderer::_double_line;

pub struct ZigZagLineFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for ZigZagLineFiller<F>
where
    F: RealNumber,
    P: BorrowMut<Vec<Vec<Point2<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut DrawOptions) -> OpSet<F> {
        let mut gap = o.hachure_gap.map(_c::<F>).unwrap_or_else(|| _c::<F>(-1.0));
        if gap < F::zero() {
            gap = o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c::<F>(1.0)) * _c::<F>(4.0);
        }
        gap = gap.max(_c::<F>(0.1));

        let mut zig_zag_offset = o
            .zigzag_offset
            .map(_c::<F>)
            .unwrap_or_else(|| _c::<F>(-1.0));
        if zig_zag_offset < F::zero() {
            zig_zag_offset = gap;
        }
        o.set_hachure_gap(Some(_to_f32(gap + zig_zag_offset)));
        let lines = polygon_hachure_lines(polygon_list.borrow_mut(), o);
        OpSet {
            op_set_type: OpSetType::FillSketch,
            ops: ZigZagLineFiller::zig_zag_lines(&lines, zig_zag_offset, o),
            size: None,
            path: None,
        }
    }
}

impl<F: RealNumber> ZigZagLineFiller<F> {
    pub fn new() -> Self {
        ZigZagLineFiller {
            _phantom: PhantomData,
        }
    }

    fn zig_zag_lines(lines: &[Line<F>], zig_zag_offset: F, o: &mut DrawOptions) -> Vec<Op<F>> {
        let mut ops = vec![];
        for line in lines.iter() {
            let length = line.length();
            let count = length / (_c::<F>(2.0) * zig_zag_offset);
            let mut p1 = line.start_point;
            let mut p2 = line.end_point;
            if p1.x > p2.x {
                p1 = line.end_point;
                p2 = line.start_point;
            }

            let alpha = ((p2.y - p1.y) / (p2.x - p1.x)).atan();

            for i in 0..(_to_u64(count)) {
                let lstart = _c::<F>(i as f32) * _c::<F>(2.0) * zig_zag_offset;
                let lend = _c::<F>((i + 1) as f32) * _c::<F>(2.0) * zig_zag_offset;
                let dz = (zig_zag_offset.powi(2) * _c::<F>(2.0)).sqrt();
                let start: Point2<F> =
                    Point2::new(p1.x + lstart * alpha.cos(), p1.y + lstart * alpha.sin());
                let end: Point2<F> =
                    Point2::new(p1.x + lend * alpha.cos(), p1.y + lend * alpha.sin());
                let middle: Point2<F> = Point2::new(
                    start.x + dz * (alpha + _c::<F>(get_pi::<f32>() / 4.0)).cos(),
                    start.y + dz * (alpha + _c::<F>(get_pi::<f32>() / 4.0)).sin(),
                );
                ops.extend(_double_line(start.x, start.y, middle.x, middle.y, o, false));

                ops.extend(_double_line(middle.x, middle.y, end.x, end.y, o, false));
            }
        }
        ops
    }
}
