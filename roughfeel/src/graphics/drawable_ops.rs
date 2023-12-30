use euclid::default::Point2D;
use euclid::Trig;
use num_traits::Float;

use super::drawable::OpSetTrait;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum OpType {
    Move,
    BCurveTo,
    LineTo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpSetType {
    Path,
    FillPath,
    FillSketch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Op<F: Float + Trig> {
    pub op: OpType,
    pub data: Vec<F>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpSet<F: Float + Trig> {
    pub op_set_type: OpSetType,
    pub ops: Vec<Op<F>>,
    pub size: Option<Point2D<F>>,
    pub path: Option<String>,
}

impl<F: Float + Trig> OpSetTrait for OpSet<F> {
    type F = F;
}
