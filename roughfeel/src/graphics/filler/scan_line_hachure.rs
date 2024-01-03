use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::marker::PhantomData;

use nalgebra::{Point2, Scalar};
use nalgebra_glm::RealNumber;
use num_traits::{Float, FromPrimitive};

use super::traits::PatternFiller;
use crate::graphics::{_c, _to_u64, _to_f64};
use crate::graphics::drawable::DrawOptions;
use crate::graphics::drawable_ops::OpSet;
use crate::graphics::geometry::{rotate_lines, rotate_points, Line};

#[derive(Clone)]
struct EdgeEntry<F: RealNumber> {
    pub(crate) ymin: F,
    pub(crate) ymax: F,
    pub(crate) x: F,
    pub(crate) islope: F,
}

impl<F: RealNumber> std::fmt::Display for EdgeEntry<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&format!(
            "ymin={} ymax={} x={} islope={}",
            _to_f64(self.ymin),
            _to_f64(self.ymax),
            _to_f64(self.x),
            _to_f64(self.islope)
        ));
    }
}

struct ActiveEdgeEntry<F: RealNumber> {
    pub(crate) s: F,
    pub(crate) edge: EdgeEntry<F>,
}

pub fn polygon_hachure_lines<F: RealNumber>(
    polygon_list: &mut Vec<Vec<Point2<F>>>,
    options: &DrawOptions,
) -> Vec<Line<F>> {
    let angle = options.hachure_angle.unwrap_or(0.0) + 90.0;
    let mut gap = options.hachure_gap.unwrap_or(0.0);
    if gap < 0.0 {
        gap = options.stroke_width.unwrap_or(0.0) * 4.0;
    }

    gap = f32::max(gap, 0.1);

    let center = Point2::new(_c(0.0), _c(0.0));
    if angle != 0.0 {
        polygon_list
            .iter_mut()
            .for_each(|polygon| *polygon = rotate_points(polygon, &center, _c(angle)))
    }

    let mut lines = straight_hachure_lines(polygon_list, _c(gap));

    if angle != 0.0 {
        polygon_list
            .iter_mut()
            .for_each(|polygon| *polygon = rotate_points(polygon, &center, _c(-angle)));
        lines = rotate_lines(&lines, &center, _c(-angle));
    }

    return lines;
}

fn straight_hachure_lines<F: Scalar>(polygon_list: &mut [Vec<Point2<F>>], gap: F) -> Vec<Line<F>>
where
    F: RealNumber,
{
    let mut vertex_array: Vec<Vec<Point2<F>>> = vec![];
    for polygon in polygon_list.iter_mut() {
        if polygon.first() != polygon.last() {
            polygon.push(
                *polygon
                    .first()
                    .expect("can not get first element of polygon"),
            );
        }
        if polygon.len() > 2 {
            vertex_array.push(polygon.clone());
        }
    }

    let mut lines: Vec<Line<F>> = vec![];
    let gap = F::max(gap, _c(0.1));

    // create sorted edges table
    let mut edges: Vec<EdgeEntry<F>> = vec![];

    for vertices in vertex_array.iter() {
        let mut edge_extension = vertices[..]
            .windows(2)
            .filter_map(|w| {
                let p1 = w[0];
                let p2 = w[1];
                if p1.y != p2.y {
                    let ymin = F::min(p1.y, p2.y);
                    Some(EdgeEntry {
                        ymin,
                        ymax: F::max(p1.y, p2.y),
                        x: if ymin == p1.y { p1.x } else { p2.x },
                        islope: (p2.x - p1.x) / (p2.y - p1.y),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<EdgeEntry<F>>>();

        edges.append(&mut edge_extension);
    }

    edges.sort_by(|e1, e2| {
        if e1.ymin < e2.ymin {
            Ordering::Less
        } else if e1.ymin > e2.ymin {
            Ordering::Greater
        } else if e1.x < e2.x {
            Ordering::Less
        } else if e1.x > e2.x {
            Ordering::Greater
        } else if e1.ymax == e2.ymax {
            Ordering::Equal
        } else {
            let ordering = (e1.ymax - e2.ymax) / (e1.ymax - e2.ymax).abs();
            if ordering > _c(0.0) {
                Ordering::Greater
            } else if ordering < _c(0.0) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    });

    if edges.is_empty() {
        return lines;
    }

    let mut active_edges: Vec<ActiveEdgeEntry<F>> = Vec::new();
    let mut y = edges.first().unwrap().ymin;

    loop {
        if !edges.is_empty() {
            let ix = edges
                .iter()
                .enumerate()
                .find(|(_ind, v)| v.ymin > y)
                .map(|(ind, _v)| ind);

            if let Some(indx) = ix {
                let removed_elements = edges.splice(0..indx, vec![]);

                removed_elements
                    .into_iter()
                    .for_each(|ee| active_edges.push(ActiveEdgeEntry { s: y, edge: ee }));
            } else {
                let removed_elements = edges.splice(0..edges.len(), vec![]);

                removed_elements
                    .into_iter()
                    .for_each(|ee| active_edges.push(ActiveEdgeEntry { s: y, edge: ee }));
            }
        }

        active_edges.retain(|ae| ae.edge.ymax > y);

        active_edges.sort_by(|ae1, ae2| {
            if ae1.edge.x == ae2.edge.x {
                Ordering::Equal
            } else {
                let ratio = (ae1.edge.x - ae2.edge.x) / (ae1.edge.x - ae2.edge.x).abs();
                if ratio > _c(0.0) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        });
        if active_edges.len() > 1 {
            active_edges[..].chunks(2).for_each(|ae| {
                let ce = &ae[0];
                let ne = &ae[1];
                lines.push(Line::from(&[
                    Point2::new(ce.edge.x, y),
                    Point2::new(ne.edge.x, y),
                ]));
            });
        }

        y = y + gap;
        active_edges.iter_mut().for_each(|ae| {
            ae.edge.x = ae.edge.x + (gap * ae.edge.islope);
        });
        if edges.is_empty() && active_edges.is_empty() {
            break;
        }
    }

    return lines;
}

pub struct ScanlineHachureFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for ScanlineHachureFiller<F>
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
        let ops = ScanlineHachureFiller::render_lines(lines, o);
        OpSet {
            op_set_type: crate::graphics::drawable_ops::OpSetType::FillSketch,
            ops: ops,
            size: None,
            path: None,
        }
    }
}

impl<F: RealNumber + FromPrimitive> ScanlineHachureFiller<F> {
    pub fn new() -> Self {
        ScanlineHachureFiller {
            _phantom: PhantomData,
        }
    }

    fn render_lines(
        lines: Vec<Line<F>>,
        o: &mut DrawOptions,
    ) -> Vec<crate::graphics::drawable_ops::Op<F>> {
        let mut ops: Vec<crate::graphics::drawable_ops::Op<F>> = vec![];
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

#[cfg(test)]
mod test {
    use nalgebra::Point2;

    use crate::graphics::geometry::Line;

    #[test]
    fn straight_hachure_lines() {
        let mut input = vec![vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 1.0),
            Point2::new(1.0, 1.0),
            Point2::new(1.0, 0.0),
        ]];
        let expected = vec![
            Line::from(&[Point2::new(0.0, 0.0), Point2::new(1.0, 0.0)]),
            Line::from(&[
                Point2::new(0.0, 0.10000000149011612),
                Point2::new(1.0, 0.10000000149011612),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.20000000298023224),
                Point2::new(1.0, 0.20000000298023224),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.30000000447034836),
                Point2::new(1.0, 0.30000000447034836),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.4000000059604645),
                Point2::new(1.0, 0.4000000059604645),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.5000000074505806),
                Point2::new(1.0, 0.5000000074505806),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.6000000089406967),
                Point2::new(1.0, 0.6000000089406967),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.7000000104308128),
                Point2::new(1.0, 0.7000000104308128),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.800000011920929),
                Point2::new(1.0, 0.800000011920929),
            ]),
            Line::from(&[
                Point2::new(0.0, 0.9000000134110451),
                Point2::new(1.0, 0.9000000134110451),
            ]),
        ];
        let result = super::straight_hachure_lines(&mut input, 0.1);
        assert_eq!(expected, result);
    }
}
