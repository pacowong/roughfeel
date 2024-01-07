use std::fmt::Display;

use nalgebra::Point2;
use nalgebra_glm::RealNumber;
use points_on_curve::{points_on_bezier_curves, simplify};
use svg_path_ops::{absolutize, normalize};
use svgtypes::{PathParser, PathSegment};

use crate::graphics::{_c, _cc};

pub fn points_on_path<F>(
    path: String,
    tolerance: Option<F>,
    distance: Option<F>,
) -> Vec<Vec<Point2<F>>>
where
    F: RealNumber + Display,
{
    let path_parser = PathParser::from(path.as_ref());
    let path_segments: Vec<PathSegment> = path_parser.flatten().collect();
    let normalized_segments = normalize(absolutize(path_segments.iter()));
    // normalized_segments
    //     .by_ref()
    //     .for_each(|a| print_line_segment(&a));
    let mut sets: Vec<Vec<Point2<F>>> = vec![];
    let mut current_points: Vec<Point2<F>> = vec![];
    let mut start = Point2::new(_c::<F>(0.0), _c::<F>(0.0));
    let mut pending_curve: Vec<Point2<F>> = vec![];

    let append_pending_curve = |current_points: &mut Vec<Point2<F>>,
                                pending_curve: &mut Vec<Point2<F>>| {
        if pending_curve.len() >= 4 {
            current_points.append(&mut points_on_bezier_curves(
                &pending_curve[..],
                tolerance.unwrap_or(_c(0.0)),
                None,
            ));
        }
        pending_curve.clear();
    };

    let mut append_pending_points =
        |current_points: &mut Vec<Point2<F>>, pending_curve: &mut Vec<Point2<F>>| {
            {
                append_pending_curve(current_points, pending_curve);
            }
            if !current_points.is_empty() {
                sets.push(current_points.clone());
                current_points.clear();
            }
        };

    for segment in normalized_segments {
        match segment {
            PathSegment::MoveTo { abs: true, x, y } => {
                append_pending_points(&mut current_points, &mut pending_curve);
                start = Point2::new(_cc::<F>(x), _cc::<F>(y));
                current_points.push(start);
            }
            PathSegment::LineTo { abs: true, x, y } => {
                append_pending_curve(&mut current_points, &mut pending_curve);
                current_points.push(Point2::new(_cc::<F>(x), _cc::<F>(y)));
            }
            PathSegment::CurveTo {
                abs: true,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                if pending_curve.is_empty() {
                    let last_point = if !current_points.is_empty() {
                        current_points.last().unwrap()
                    } else {
                        &start
                    };
                    pending_curve.push(*last_point);
                }
                pending_curve.push(Point2::new(_cc::<F>(x1), _cc::<F>(y1)));
                pending_curve.push(Point2::new(_cc::<F>(x2), _cc::<F>(y2)));
                pending_curve.push(Point2::new(_cc::<F>(x), _cc::<F>(y)));
            }
            PathSegment::ClosePath { abs: true } => {
                append_pending_curve(&mut current_points, &mut pending_curve);
                current_points.push(start);
            }
            _ => panic!("unexpected  path segment"),
        }
    }

    append_pending_points(&mut current_points, &mut pending_curve);

    if let Some(dst) = distance {
        let mut out = vec![];
        for set in sets.iter() {
            let simplified_set = simplify(set, dst);
            if !simplified_set.is_empty() {
                out.push(simplified_set);
            }
        }
        out
    } else {
        sets
    }
}
