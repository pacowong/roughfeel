//! This example plots bezier curve and computed points on it

// use euclid::{default, point2};
use nalgebra::{Point2, Scalar};
use piet::kurbo::{Circle, CubicBez, Point, TranslateScale, Vec2};
use piet::{Color, RenderContext};
use piet_common::kurbo::Rect;
use piet_common::Device;
use piet_cairo::CairoRenderContext;
use points_on_curve::points_on_bezier_curves;

const WIDTH: usize = 740;
const HEIGHT: usize = 500;
/// For now, assume pixel density (dots per inch)
const DPI: f64 = 96.;

/// Feature "png" needed for save_to_file() and it's disabled by default for optional dependencies
/// cargo run --example mondrian --features png

fn point2_to_tuple<T: Scalar + Copy>(p: &Point2<T>) -> (T, T) {
    (p.x, p.y)
}

fn main() {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut rc = bitmap.render_context();

    let background_color = Color::from_hex_str("96C0B7").unwrap();
    let stroke_color = Color::from_hex_str("725752").unwrap();
    let sketch_color = Color::from_hex_str("FEF6C9").unwrap();

    let input = vec![
        Point2::new(70.0, 240.0),
        Point2::new(145.0, 60.0),
        Point2::new(275.0, 90.0),
        Point2::new(300.0, 230.0),
    ];
    let result_015 = points_on_bezier_curves(&input, 0.2, Some(0.15));

    rc.fill(
        Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
        &background_color,
    );

    let original_curve = CubicBez::new(
        Point::from(point2_to_tuple(&input[0])),
        Point::from(point2_to_tuple(&input[1])),
        Point::from(point2_to_tuple(&input[2])),
        Point::from(point2_to_tuple(&input[3])),
    );

    let dpi_multiplier = 0.05;

    rc.stroke(original_curve, &stroke_color, 0.01 * DPI);

    result_015.iter().for_each(|p| {
        let circle = Circle::new(Point::from((p.x, p.y)), 1.0);
        rc.stroke(circle, &sketch_color, dpi_multiplier * DPI);
    });

    let translation = TranslateScale::translate(Vec2::new(370.0, 0.0));
    let result_075 = points_on_bezier_curves(&input, 0.2, Some(0.75));
    draw_point_on_curve(&original_curve, result_075, &translation, &mut rc);

    let translation = TranslateScale::translate(Vec2::new(0.0, 250.0));
    let result_15 = points_on_bezier_curves(&input, 0.2, Some(1.5));
    draw_point_on_curve(&original_curve, result_15, &translation, &mut rc);

    let translation = TranslateScale::translate(Vec2::new(370.0, 250.0));
    let result_30 = points_on_bezier_curves(&input, 0.2, Some(3.0));
    draw_point_on_curve(&original_curve, result_30, &translation, &mut rc);

    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("tolerance.png")
        .expect("file save error");
}

fn draw_point_on_curve(
    original_curve: &CubicBez,
    estimation: Vec<Point2<f64>>,
    translation: &TranslateScale,
    rc: &mut impl RenderContext,
) {
    let dpi_multiplier = 0.05;

    let curve = *translation * *original_curve;
    let stroke_color = Color::from_hex_str("725752").unwrap();
    rc.stroke(curve, &stroke_color, 0.01 * DPI);

    estimation.iter().for_each(|p| {
        let mut circle = Circle::new(Point::from(point2_to_tuple(p)), 1.0);
        circle = *translation * circle;
        rc.stroke(
            circle,
            &Color::from_hex_str("FEF6C9").unwrap(),
            dpi_multiplier * DPI,
        );
    });
}
