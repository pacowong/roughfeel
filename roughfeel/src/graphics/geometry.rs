// Copy from https://github.com/orhanbalci/rough-rs/blob/main/roughr/src/geometry.rs
use nalgebra::{Point2, Vector2, Rotation2};
use nalgebra_glm::RealNumber;

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
        assert_eq!(points.len(), 2);
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
    pub fn rotate(&mut self, center: &Point2<F>, degrees: F) -> &mut Self {
        let rotated_end_points = rotate_points(&[self.start_point, self.end_point], center, degrees);
        self.start_point = rotated_end_points[0];
        self.end_point = rotated_end_points[1];
        self
    }
}

fn degree_to_radians<F: RealNumber>(degrees: F) -> F {
    degrees / F::from_f64(180.0 / 3.141592653589793238).unwrap()
}

pub fn rotate_points<F: RealNumber>(
    points: &[Point2<F>],
    center: &Point2<F>,
    degrees: F,
) -> Vec<Point2<F>> {
    let angle = degree_to_radians(degrees);
    let translation_to_center = Vector2::new(center.x, center.y);
    let rot_mat = Rotation2::new(angle);
    // Translate to origin -> Rotate by angle (rad.) -> Translate to the original center
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
    // https://fontforge.org/docs/techref/bezier.html#converting-truetype-to-postscript
    let scaling_factor = F::from_f64(2.0/3.0).unwrap();
    let cubic_pt1 = bezier_quadratic.start + (bezier_quadratic.cp - bezier_quadratic.start) * scaling_factor;
    let cubic_pt2 = bezier_quadratic.end + (bezier_quadratic.cp - bezier_quadratic.end) * scaling_factor;
    BezierCubic {
        start: bezier_quadratic.start,
        cp1: cubic_pt1,
        cp2: cubic_pt2,
        end: bezier_quadratic.end,
    }
}

#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use nalgebra::Point2;

    use super::{BezierCubic, BezierQuadratic, convert_bezier_quadratic_to_cubic};

    #[test]
    fn line_length() {
        let l = super::Line::from(&[Point2::new(1.0, 1.0), Point2::new(2.0, 2.0)]);
        assert_eq!(l.length(), f32::sqrt(2.0));
    }

    #[test]
    fn line_rotate() {
        let mut l = super::Line::from(&[Point2::new(1.0, 1.0), Point2::new(2.0, 2.0)]);
        l.rotate(&Point2::new(0.0, 0.0), 45.0);
        assert!(relative_eq!(l.start_point, Point2::new(0.0, f64::sqrt(2.0)), epsilon = 1.0e-7));
        assert!(relative_eq!(l.end_point, Point2::new(0.0, 2.0*f64::sqrt(2.0)), epsilon = 1.0e-7));

        l.rotate(&Point2::new(0.0, 0.0), -35.0).rotate(&Point2::new(0.0, 0.0), -10.0);
        assert!(relative_eq!(l.start_point, Point2::new(1.0, 1.0), epsilon = 1.0e-7));
        assert!(relative_eq!(l.end_point, Point2::new(2.0, 2.0), epsilon = 1.0e-7));

        l.rotate(&Point2::new(1.0, 10.0), 45.0);
        assert!(relative_eq!(l.start_point, Point2::new(7.363961030678928, 3.636038969321072), epsilon = 1.0e-7));
        assert!(relative_eq!(l.end_point, Point2::new(7.363961030678928, 5.050252531694167), epsilon = 1.0e-7));
    }


}
