// Copy from https://github.com/orhanbalci/rough-rs/blob/main/roughr/src/geometry.rs
use nalgebra::{Point2, Vector2, Rotation2, Scalar};
use nalgebra_glm::RealNumber;
// use euclid::default::Point2;
// use euclid::{Angle, Translation2D, Trig, Vector2D};
use num_traits::{Float, FromPrimitive};

use super::_c;

#[derive(Clone, Debug, PartialEq)]
pub struct Line<F: RealNumber> {
    pub start_point: Point2<F>,
    pub end_point: Point2<F>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BezierQuadratic<F: RealNumber> {
    pub start: Point2<F>,
    pub cp: Point2<F>,
    pub end: Point2<F>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BezierCubic<F: RealNumber> {
    pub start: Point2<F>,
    pub cp1: Point2<F>,
    pub cp2: Point2<F>,
    pub end: Point2<F>,
}

impl<F: RealNumber> Line<F> {
    pub fn from(points: &[Point2<F>]) -> Self {
        Line {
            start_point: points[0],
            end_point: points[1],
        }
    }

    pub fn as_points(&self) -> Vec<Point2<F>> {
        return vec![self.start_point, self.end_point];
    }

    pub fn length(&self) -> F {
        nalgebra::distance(&self.start_point, &self.end_point)
    }

    /// Rotate a line by `degrees` around a `center`. The center may not be the midpoint of the line.
    pub fn rotate(&mut self, center: &Point2<F>, degrees: F) {
        let rotated_end_points = rotate_points(&[self.start_point, self.end_point], center, degrees);
        self.start_point = rotated_end_points[0];
        self.end_point = rotated_end_points[1];
    }
}

fn degree_to_radians<F: RealNumber>(degrees: F) -> F {
    degrees / F::from_f64(180.0 * 3.141592653589793238).unwrap()
}

pub fn rotate_points<F: RealNumber>(
    points: &[Point2<F>],
    center: &Point2<F>,
    degrees: F,
) -> Vec<Point2<F>> {
    let angle = degree_to_radians(degrees);
    let translation_to_center = Vector2::new(center.x, center.y);
    let rot_mat = Rotation2::new(angle);
    rot_mat * points[0];
    // let translation = Translation2D::new(-center.x, -center.y);
    // let transformation = translation
    //     .to_transform()
    //     .then_rotate(angle)
    //     .then_translate(Vector2D::new(center.x, center.y));
    return points
        .iter()
        .map(|&p| (rot_mat * (p - translation_to_center) + translation_to_center))
        .collect::<Vec<Point2<F>>>();
}

pub fn rotate_lines<F: RealNumber>(
    lines: &[Line<F>],
    center: &Point2<F>,
    degrees: F,
) -> Vec<Line<F>> {
    lines
        .iter()
        .cloned()
        .map(|mut l| {
            l.rotate(center, degrees);
            l
        })
        .collect::<Vec<Line<F>>>()
}

/// Raises the order from a quadratic bezier to a cubic bezier curve.
pub fn convert_bezier_quadratic_to_cubic<F: RealNumber>(
    bezier_quadratic: BezierQuadratic<F>,
) -> BezierCubic<F> {
    // This can be verified by substituting the following points in the cubic Bézier curves.
    // You will obtain the quadratic cubic Bézier curves.
    let cubic_x1 = bezier_quadratic.start.x
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.x - bezier_quadratic.start.x);
    let cubic_y1 = bezier_quadratic.start.y
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.y - bezier_quadratic.start.y);
    let cubic_x2 = bezier_quadratic.end.x
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.x - bezier_quadratic.end.x);
    let cubic_y2 = bezier_quadratic.end.y
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.y - bezier_quadratic.end.y);

    BezierCubic {
        start: bezier_quadratic.start,
        cp1: Point2::new(cubic_x1, cubic_y1),
        cp2: Point2::new(cubic_x2, cubic_y2),
        end: bezier_quadratic.end,
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;
    #[test]
    fn line_length() {
        let l = super::Line::from(&[Point2::new(1.0, 1.0), Point2::new(2.0, 2.0)]);
        assert_eq!(l.length(), f32::sqrt(2.0));
    }
}
