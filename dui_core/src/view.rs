use dui_macros::multi;
use vello::{
    kurbo::{Affine, Rect, Shape},
    peniko::Brush,
};

use crate::drawing::DrawingContext;

pub trait Element {
    type Body: View;

    fn body(&self) -> Self::Body;
}

pub trait ElementIterator {
    fn len(&self) -> usize;
    fn layout_at(&self, available_rect: Rect, index: usize) -> Rect;
    fn draw_at(&self, dctx: &mut DrawingContext, index: usize);
}

pub trait View {
    fn layout(&self, available_rect: Rect) -> Rect;

    fn draw(&self, dctx: &mut DrawingContext);
}

pub struct VStack<E: ElementIterator> {
    element: E,
}

impl<E: ElementIterator> VStack<E> {
    pub fn new(element: impl Into<E>) -> VStack<E> {
        VStack {
            element: element.into(),
        }
    }
}

impl<E: ElementIterator> View for VStack<E> {
    fn layout(&self, available_rect: Rect) -> Rect {
        let mut current_rect = available_rect;
        current_rect.y1 = 0.0;

        for i in 0..self.element.len() {
            let layout = self.element.layout_at(current_rect, i);

            current_rect.y1 += layout.height();
        }

        available_rect
    }

    fn draw(&self, _dctx: &mut DrawingContext) {}
}

impl<S: Shape> View for S {
    fn layout(&self, available_rect: Rect) -> Rect {
        self.bounding_box().intersect(available_rect)
    }

    fn draw(&self, dctx: &mut DrawingContext) {
        dctx.builder.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            &Brush::Solid(dctx.foreground_color),
            None,
            self,
        );
    }
}

multi!(Multi, 2);
multi!(Multi, 3);
multi!(Multi, 4);
multi!(Multi, 5);
multi!(Multi, 6);
multi!(Multi, 7);
multi!(Multi, 8);
multi!(Multi, 9);
multi!(Multi, 10);
multi!(Multi, 11);
multi!(Multi, 12);
multi!(Multi, 13);
multi!(Multi, 14);
