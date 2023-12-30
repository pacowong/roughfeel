use piet::{kurbo::Rect, Color, RenderContext};
use piet_common::kurbo::{Point, Size};
use piet_common::Device;
// use rand::{prelude::*, random};
// use rand_distr::Normal;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// For now, assume pixel density (dots per inch)
const DPI: f64 = 96.;

/// Feature "png" needed for save_to_file() and it's disabled by default for optional dependencies
/// cargo run --example mondrian --features png
fn main() {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut rc = bitmap.render_context();
    // let options = OptionsBuilder::default()
    //     .stroke(Srgba::from_components((114u8, 87u8, 82u8, 255u8)).into_format())
    //     .fill(Srgba::from_components((254u8, 246u8, 201u8, 255)).into_format())
    //     .fill_style(FillStyle::ZigZagLine)
    //     .fill_weight(DPI * 0.01)
    //     .build()
    //     .unwrap();

    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("temp-image.png")
        .expect("file save error");
}
