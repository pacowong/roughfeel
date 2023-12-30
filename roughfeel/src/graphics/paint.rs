#[derive(Clone, PartialEq, Debug)]
pub enum FillStyle {
    Solid,
    Hachure,
    ZigZag,
    CrossHatch,
    Dots,
    Dashed,
    ZigZagLine,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Options for angled joins in strokes.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LineJoin {
    Miter { limit: f64 },
    Round,
    Bevel,
}
impl LineJoin {
    pub const DEFAULT_MITER_LIMIT: f64 = 10.0;
}
impl Default for LineJoin {
    fn default() -> Self {
        LineJoin::Miter {
            limit: LineJoin::DEFAULT_MITER_LIMIT,
        }
    }
}
