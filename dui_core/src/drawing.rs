use vello::{kurbo::Rect, peniko::Color, SceneBuilder};

use crate::view::{Element, View};

pub struct DrawingContext<'a> {
    pub builder: SceneBuilder<'a>,

    pub background_color: Color,
    pub foreground_color: Color,

    pub bounding: Rect,
}

pub fn draw<'a>(dctx: &mut DrawingContext<'a>, view: impl Element) {
    view.body().draw(dctx)
}
