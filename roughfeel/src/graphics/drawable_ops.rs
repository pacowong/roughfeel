use nalgebra::Point2;
use nalgebra_glm::RealNumber;

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

/// A unified data structure that stores all drawing operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Op<F: RealNumber> {
    //Paco: SIMD?
    pub op: OpType,
    pub data: Vec<F>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpSet<F: RealNumber> {
    pub op_set_type: OpSetType,
    pub ops: Vec<Op<F>>,
    pub size: Option<Point2<F>>,
    pub path: Option<String>,
}

impl<F: RealNumber> OpSetTrait for OpSet<F> {
    type F = F;
}
