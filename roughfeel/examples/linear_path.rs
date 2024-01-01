//! This example shows painting a rough circle using common-piet crate and
//! kurbo rough shape generator

// extern crate roughfeel;

use euclid::{default, Point2D};
use palette::Srgba;
use piet::{Color, RenderContext};
use piet_common::kurbo::Rect;
use piet_common::Device;
// use crate::graphics::drawable_maker::DrawOptionsBuilder;
use roughfeel::graphics::drawable::{DrawOptionsBuilder, RoughlyDrawable};
use roughfeel::graphics::drawable_ops::OpSet;
use roughfeel::renderer_engine::kurbo_drawable::{KurboDrawable, KurboOpSet};
use roughfeel::renderer_engine::kurbo_drawable_maker::KurboDrawableMaker;

use roughfeel::graphics::drawable_maker::{Generator, RoughlyDrawableMaker};
use roughfeel::graphics::paint::FillStyle;
use roughfeel::*;

// use rough_piet::KurboGenerator;
// use roughr::core::{FillStyle, OptionsBuilder};

const WIDTH: usize = 100;
const HEIGHT: usize = 50;
/// For now, assume pixel density (dots per inch)
const DPI: f32 = 96.;

/// Feature "png" needed for save_to_file() and it's disabled by default for optional dependencies
/// cargo run --example mondrian --features png
fn main() {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut rc = bitmap.render_context();
    let options = DrawOptionsBuilder::default()
        .stroke(Srgba::from_components((114u8, 87u8, 82u8, 255u8)).into_format())
        .fill(Srgba::from_components((254u8, 246u8, 201u8, 255u8)).into_format())
        .fill_style(FillStyle::Hachure)
        .fill_weight(DPI * 0.01)
        .build()
        .unwrap();
    let generator = KurboDrawableMaker::<f32, KurboDrawable<f32>>::new(
        Generator::<OpSet<f32>>::new(options.clone()),
    );
    //::default{}; //Generator::<f32, f32, KurboOpSet<f32>>::new(options);//::default();//::new(options);
    let points = [
        Point2D::new(0.0, HEIGHT as f32 / 2.0),
        Point2D::new(WIDTH as f32 / 2.0, HEIGHT as f32),
        Point2D::new(WIDTH as f32, HEIGHT as f32 / 2.0),
        Point2D::new(WIDTH as f32 / 2.0, 0.0),
    ];
    let linear_path = generator.linear_path(&points, true, &Some(options));
    let background_color = Color::from_hex_str("96C0B7").unwrap();

    rc.fill(
        Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
        &background_color,
    );
    linear_path.draw(&mut rc);
    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("linear_path.png")
        .expect("file save error");
}
