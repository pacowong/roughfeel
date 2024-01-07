use derive_builder::Builder;
use nalgebra_glm::RealNumber;
use palette::Srgba;
use rand::{random, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::{
    drawable_ops::OpSet,
    paint::{FillStyle, LineCap, LineJoin},
};

pub struct PathInfo {
    pub d: String,
    pub stroke: Option<Srgba>,
    pub stroke_width: Option<f32>,
    pub fill: Option<Srgba>,
}

#[derive(Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DrawOptions {
    #[builder(default = "Some(2.0)")]
    pub max_randomness_offset: Option<f32>,
    #[builder(default = "Some(1.0)")]
    pub roughness: Option<f32>,
    #[builder(default = "Some(2.0)")]
    pub bowing: Option<f32>,
    #[builder(default = "Some(Srgba::new(0.0, 0.0, 0.0, 1.0))")]
    pub stroke: Option<Srgba>,
    #[builder(default = "Some(1.0)")]
    pub stroke_width: Option<f32>,
    #[builder(default = "Some(0.95)")]
    pub curve_fitting: Option<f32>,
    #[builder(default = "Some(0.0)")]
    pub curve_tightness: Option<f32>,
    #[builder(default = "Some(9.0)")]
    pub curve_step_count: Option<f32>,
    #[builder(default = "None")]
    pub fill: Option<Srgba>,
    #[builder(default = "None")]
    pub fill_style: Option<FillStyle>,
    #[builder(default = "Some(-1.0)")]
    pub fill_weight: Option<f32>,
    #[builder(default = "Some(-41.0)")]
    pub hachure_angle: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub hachure_gap: Option<f32>,
    #[builder(default = "Some(1.0)")]
    pub simplification: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub dash_offset: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub dash_gap: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub zigzag_offset: Option<f32>,
    #[builder(default = "Some(345_u64)")]
    pub seed: Option<u64>,
    #[builder(default = "None")]
    pub stroke_line_dash: Option<Vec<f64>>,
    #[builder(default = "None")]
    pub stroke_line_dash_offset: Option<f64>,
    #[builder(default = "None")]
    pub line_cap: Option<LineCap>,
    #[builder(default = "None")]
    pub line_join: Option<LineJoin>,
    #[builder(default = "None")]
    pub fill_line_dash: Option<Vec<f64>>,
    #[builder(default = "None")]
    pub fill_line_dash_offset: Option<f64>,
    #[builder(default = "Some(false)")]
    pub disable_multi_stroke: Option<bool>,
    #[builder(default = "Some(false)")]
    pub disable_multi_stroke_fill: Option<bool>,
    #[builder(default = "Some(false)")]
    pub preserve_vertices: Option<bool>,
    #[builder(default = "None")]
    pub fixed_decimal_place_digits: Option<f32>,
    #[builder(default = "None")]
    pub randomizer: Option<rand_chacha::ChaCha8Rng>,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self {
            max_randomness_offset: Some(2.0),
            roughness: Some(1.0),
            bowing: Some(2.0),
            stroke: Some(Srgba::new(0.0, 0.0, 0.0, 1.0)),
            stroke_width: Some(1.0),
            curve_tightness: Some(0.0),
            curve_fitting: Some(0.95),
            curve_step_count: Some(9.0),
            fill: None,
            fill_style: None,
            fill_weight: Some(-1.0),
            hachure_angle: Some(-41.0),
            hachure_gap: Some(-1.0),
            dash_offset: Some(-1.0),
            dash_gap: Some(-1.0),
            zigzag_offset: Some(-1.0),
            seed: Some(345_u64),
            disable_multi_stroke: Some(false),
            disable_multi_stroke_fill: Some(false),
            preserve_vertices: Some(false),
            simplification: Some(1.0),
            stroke_line_dash: None,
            stroke_line_dash_offset: None,
            line_cap: None,
            line_join: None,
            fill_line_dash: None,
            fill_line_dash_offset: None,
            fixed_decimal_place_digits: None,
            randomizer: None,
        }
    }
}

impl DrawOptions {
    pub fn random(&mut self) -> f64 {
        match &mut self.randomizer {
            Some(r) => r.gen(),
            None => {
                let s = match self.seed {
                    Some(s) => s,
                    None => random(),
                };
                self.randomizer = Some(ChaCha8Rng::seed_from_u64(s));
                match &mut self.randomizer {
                    Some(r1) => r1.gen(),
                    None => 0.0,
                }
            }
        }
    }

    pub fn set_hachure_angle(&mut self, angle: Option<f32>) -> &mut Self {
        self.hachure_angle = angle;
        self
    }

    pub fn set_hachure_gap(&mut self, gap: Option<f32>) -> &mut Self {
        self.hachure_gap = gap;
        self
    }
}

pub trait OpSetTrait {
    type F: RealNumber;
}

pub trait Drawable<OpSetT: OpSetTrait>
where
    OpSetT::F: RealNumber,
{
    // A drawable is a general concept for a graphic that can be drawn to the screen.
    fn draw(shape: String, options: DrawOptions, sets: Vec<OpSetT>) -> Self;
}

pub struct RoughlyDrawable<OpSetT: OpSetTrait>
where
    OpSetT::F: RealNumber,
{
    pub shape: String,
    pub options: DrawOptions,
    pub opsets: Vec<OpSetT>,
}

impl<AF: RealNumber> Drawable<OpSet<AF>> for RoughlyDrawable<OpSet<AF>> {
    fn draw(
        shape: String,
        options: DrawOptions,
        sets: Vec<OpSet<AF>>,
    ) -> RoughlyDrawable<OpSet<AF>> {
        Self {
            shape: shape.into(),
            options: options.clone(),
            opsets: Vec::from_iter(sets.iter().cloned()),
        }
    }
}
