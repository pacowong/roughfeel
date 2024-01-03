use std::borrow::BorrowMut;
use std::marker::PhantomData;

use nalgebra::{Point2, Scalar};
use nalgebra_glm::RealNumber;
use num_traits::{Float, FromPrimitive};

use super::scan_line_hachure::ScanlineHachureFiller;
use super::traits::PatternFiller;
use crate::graphics::_c;
use crate::graphics::drawable::DrawOptions;
use crate::graphics::drawable_ops::OpSet;

pub struct HatchFiller<F> {
    _phantom: PhantomData<F>,
    hachure_filler: ScanlineHachureFiller<F>,
}

impl<F, P> PatternFiller<F, P> for HatchFiller<F>
where
    F: RealNumber,
    P: BorrowMut<Vec<Vec<Point2<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut DrawOptions) -> OpSet<F> {
        let mut set1 = self
            .hachure_filler
            .fill_polygons(polygon_list.borrow_mut(), o);
        o.set_hachure_angle(o.hachure_angle.map(|a| a + 90.0));
        let set2 = self.hachure_filler.fill_polygons(polygon_list, o);
        set1.ops.extend(set2.ops);
        set1
    }
}

impl<F: RealNumber> HatchFiller<F> {
    pub fn new() -> Self {
        HatchFiller {
            _phantom: PhantomData,
            hachure_filler: ScanlineHachureFiller::new(),
        }
    }
}

impl<F: RealNumber> Default for HatchFiller<F> {
    fn default() -> Self {
        Self::new()
    }
}
