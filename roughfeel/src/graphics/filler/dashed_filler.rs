use std::borrow::BorrowMut;
use std::marker::PhantomData;

use nalgebra::{Point2, Scalar, Vector};
use nalgebra_glm::RealNumber;

use super::scan_line_hachure::polygon_hachure_lines;
use super::traits::PatternFiller;
use crate::graphics::_c;
use crate::graphics::drawable::DrawOptions;
use crate::graphics::drawable_ops::OpSet;
use crate::graphics::geometry::Line;
use crate::graphics::renderer::_double_line;

pub struct DashedFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for DashedFiller<F>
where
    F: RealNumber,
    P: BorrowMut<Vec<Vec<Point2<F>>>>,
{
    fn fill_polygons(
        &self,
        mut polygon_list: P,
        o: &mut DrawOptions,
    ) -> crate::graphics::drawable_ops::OpSet<F> {
        let lines = polygon_hachure_lines(polygon_list.borrow_mut(), o);
        let ops = DashedFiller::dashed_line(lines, o);
        OpSet {
            op_set_type: crate::graphics::drawable_ops::OpSetType::FillSketch,
            ops,
            size: None,
            path: None,
        }
    }
}
impl<'a, F: RealNumber> DashedFiller<F> {
    pub fn new() -> Self {
        DashedFiller {
            _phantom: PhantomData,
        }
    }

    fn dashed_line(
        lines: Vec<Line<F>>,
        o: &mut DrawOptions,
    ) -> Vec<crate::graphics::drawable_ops::Op<F>> {
        let dash_offset: F = o.dash_offset.map(_c).unwrap_or_else(|| _c(-1.0));
        let offset = if dash_offset < _c(0.0) {
            let hachure_gap: F = o.hachure_gap.map(_c).unwrap_or_else(|| _c(-1.0));
            if hachure_gap < _c(0.0) {
                o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c(1.0)) * _c(4.0)
            } else {
                hachure_gap
            }
        } else {
            dash_offset
        };
        let dash_gap = o.dash_gap.map(_c).unwrap_or_else(|| _c(-1.0));
        let gap: F = if dash_gap < _c(0.0) {
            let hachure_gap = o.hachure_gap.map(_c).unwrap_or_else(|| _c(-1.0));
            if hachure_gap < _c(0.0) {
                o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c(1.0)) * _c(4.0)
            } else {
                hachure_gap
            }
        } else {
            dash_gap
        };

        let mut ops = vec![];

        for line in lines.iter() {
            let length = line.length();
            let count = (length / (offset + gap)).floor();
            let start_offset = (length + gap - (count * (offset + gap))) / _c(2.0);
            let mut p1 = line.start_point;
            let mut p2 = line.end_point;
            if p1.x > p2.x {
                p1 = line.end_point;
                p2 = line.start_point;
            }
            let alpha = ((p2.y - p1.y) / (p2.x - p1.x)).atan();
            let count: f64 = nalgebra::try_convert(count).unwrap(); //count.map.try_into().unwrap();
            for i in 0..(count as u64) {
                //.try_into::<u32>::().to_u32().unwrap() {
                let lstart = F::from_u64(i).unwrap() * (offset + gap); //F::from(i).unwrap() * (offset + gap);
                let lend = lstart + offset;
                let start = Point2::<F>::new(
                    p1.x + (lstart * alpha.cos()) + (start_offset * alpha.cos()),
                    p1.y + lstart * alpha.sin() + (start_offset * alpha.sin()),
                );
                let end = Point2::<F>::new(
                    p1.x + (lend * alpha.cos()) + (start_offset * alpha.cos()),
                    p1.y + (lend * alpha.sin()) + (start_offset * alpha.sin()),
                );
                let line_ops = _double_line(start.x, start.y, end.x, end.y, o, false);
                ops.extend(line_ops);
            }
        }

        ops
    }
}
