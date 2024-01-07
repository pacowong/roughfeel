use derive_builder::Builder;
use nalgebra::{Point2, Scalar};
use nalgebra_glm::RealNumber;
use palette::Srgba;
use rand_chacha::{rand_core::block::BlockRngCore, ChaCha8Core};

use crate::graphics::drawable::DrawOptions;

use crate::graphics::drawable_ops::{OpSet, OpSetType, OpType};

pub struct Drawable2D<F: RealNumber> {
    pub shape: String,
    pub options: DrawOptions,
    pub opsets: Vec<OpSet<F>>,
}

pub struct RoughCanvasDrawer {}

pub struct Canvas2D {}

pub struct Canvas2DContext {}

pub struct RoughCanvas {
    drawer: RoughCanvasDrawer,
    canvas: Canvas2D,
    ctx: Canvas2DContext,
}

// impl RoughCanvas {
//     fn draw<F: Float + Trig>(drawable: Drawable2D<F>) {
//         for op in drawable.opsets {
//             match op.op_set_type {
//                 OpSetType::Path => {
//                     ctx.save();
//                     ctx.strokeStyle
//                 },
//                 OpSetType::FillPath => todo!(),
//                 OpSetType::FillSketch => todo!(),
//             }
//         }
//     }

//     fn _drawToContext(ctx: CanvasContext2D, drawing: OpSet, fixedDecimals: number, rule: CanvasFillRule = 'nonzero') {
//         ctx.beginPath();
//         for item in drawing.ops {
//           const data = ((typeof fixedDecimals === 'number') && fixedDecimals >= 0) ? (item.data.map((d) => +d.toFixed(fixedDecimals))) : item.data;
//           switch (item.op) {
//             case 'move':
//               ctx.moveTo(data[0], data[1]);
//               break;
//             case 'bcurveTo':
//               ctx.bezierCurveTo(data[0], data[1], data[2], data[3], data[4], data[5]);
//               break;
//             case 'lineTo':
//               ctx.lineTo(data[0], data[1]);
//               break;
//           }
//         }
//         if (drawing.type === 'fillPath') {
//           ctx.fill(rule);
//         } else {
//           ctx.stroke();
//         }
// }
