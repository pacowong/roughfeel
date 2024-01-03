use std::borrow::BorrowMut;

use nalgebra::{Point2};
use nalgebra_glm::RealNumber;
use num_traits::{Float, FromPrimitive};

use crate::graphics::drawable::DrawOptions;
use crate::graphics::drawable_ops::OpSet;

pub trait PatternFiller<F: RealNumber, P: BorrowMut<Vec<Vec<Point2<F>>>>> {
    fn fill_polygons(&self, polygon_list: P, o: &mut DrawOptions) -> OpSet<F>;
}
