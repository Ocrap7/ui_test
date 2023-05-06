use std::rc::Rc;

use dui_macros::{multi, multi_from};
use vello::{
    kurbo::{Affine, Rect},
    peniko::Brush,
};

use crate::{
    defaults::DEFAULT_SPACING,
    drawing::{DrawingContext, LayoutContext},
    layout::{get_id_manger, get_id_manger_mut},
};

pub trait Element {
    fn body(&self) -> impl Element + View;

    fn view(&self) -> impl View {
        return self.body();
    }

    fn is_leaf(&self) -> bool {
        false
    }
}

impl Element for () {
    fn body(&self) -> impl Element + View {
        ()
    }

    fn is_leaf(&self) -> bool {
        true
    }
}

impl View for () {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        Rect::ZERO
    }

    fn draw(&self, dctx: &mut DrawingContext) {}
}

pub trait ElementIterator {
    fn len(&self) -> usize;
    fn layout_at(&self, lctx: &mut LayoutContext, available_rect: Rect, index: usize) -> Rect;
    fn draw_at(&self, dctx: &mut DrawingContext, index: usize);
    fn is_leaf_at(&self, index: usize) -> bool;
}

// impl<E: Element> ElementIterator for E {
//     fn len(&self) -> usize {
//         1
//     }

//     fn layout_at(&self, available_rect: Rect, index: usize) -> Rect {
//         self.body().layout(available_rect)
//     }

//     fn draw_at(&self, dctx: &mut DrawingContext, index: usize) {
//         self.body().draw(dctx)
//     }
// }

impl<V: View> ElementIterator for V {
    fn len(&self) -> usize {
        1
    }

    fn layout_at(&self, lctx: &mut LayoutContext, available_rect: Rect, index: usize) -> Rect {
        // Rc::get_mut(&mut lctx.path).unwrap().push(0);
        lctx.path.push(0);

        let layout = self.layout(lctx, available_rect);
        // get_id_manger_mut().insert(path.clone());
        get_id_manger_mut().set_layout_full_rect(Vec::clone(&lctx.path).into(), layout);

        // Rc::get_mut(&mut lctx.path).unwrap().pop();
        lctx.path.pop();

        layout
    }

    fn draw_at(&self, dctx: &mut DrawingContext, index: usize) {
        self.draw(dctx)
    }

    fn is_leaf_at(&self, index: usize) -> bool {
        false
    }
}

pub trait View {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect;

    fn draw(&self, dctx: &mut DrawingContext);
}

pub struct VStack<E: ElementIterator> {
    spacing: f64,
    element: E,
}

impl<E: ElementIterator> VStack<E> {
    pub fn new(element: impl Into<E>) -> VStack<E> {
        VStack {
            spacing: DEFAULT_SPACING,
            element: element.into(),
        }
    }
}

multi_from!(VStack, Multi, 2);
multi_from!(VStack, Multi, 3);
multi_from!(VStack, Multi, 4);
multi_from!(VStack, Multi, 5);
multi_from!(VStack, Multi, 6);
multi_from!(VStack, Multi, 7);
multi_from!(VStack, Multi, 8);
multi_from!(VStack, Multi, 9);
multi_from!(VStack, Multi, 10);
multi_from!(VStack, Multi, 12);
multi_from!(VStack, Multi, 13);
multi_from!(VStack, Multi, 14);

impl<E: ElementIterator> Element for VStack<E> {
    fn body(&self) -> impl Element + View {
        ()
    }
}

impl<E: ElementIterator> View for VStack<E> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let mut used_rect = available_rect;
        used_rect.y1 = 0.0;

        let mut current_rect = available_rect;

        // Rc::get_mut(&mut lctx.path).unwrap().push(0);
        lctx.path.push(0);

        for i in 0..self.element.len() {
            // *Rc::get_mut(&mut lctx.path).unwrap().last_mut().unwrap() = i as u32;
            *lctx.path.last_mut().unwrap() = i as u32;

            let layout = self.element.layout_at(lctx, current_rect, i);

            used_rect.y1 += layout.height();
            current_rect.y0 += layout.height();

            if i != self.element.len() - 1 {
                used_rect.y1 += self.spacing * lctx.scale_factor;
                current_rect.y0 += self.spacing * lctx.scale_factor;
            }
        }

        // Rc::get_mut(&mut lctx.path).unwrap().pop();
        lctx.path.pop();

        get_id_manger_mut().set_layout_full_rect(Vec::clone(&lctx.path).into(), used_rect);

        used_rect
    }

    fn draw(&self, dctx: &mut DrawingContext) {
        Rc::get_mut(&mut dctx.path).unwrap().push(0);

        for i in 0..self.element.len() {
            *Rc::get_mut(&mut dctx.path).unwrap().last_mut().unwrap() = i as u32;

            self.element.draw_at(dctx, i)
        }

        Rc::get_mut(&mut dctx.path).unwrap().pop();
    }
}

impl Element for Rect {
    fn body(&self) -> impl Element + View {
        self.clone()
    }
}

impl View for Rect {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let rect = Rect {
            x0: available_rect.x0,
            y0: available_rect.y0,
            x1: available_rect.x0 + self.width() * lctx.scale_factor,
            y1: available_rect.y0 + self.height() * lctx.scale_factor,
        };

        get_id_manger_mut().set_layout_full_rect(Vec::clone(&lctx.path).into(), rect);

        rect
    }

    fn draw(&self, dctx: &mut DrawingContext) {
        let binding = get_id_manger();
        let layout = binding.get_layout(Vec::clone(&dctx.path).into());

        Rc::get_mut(&mut dctx.builder).unwrap().fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            &Brush::Solid(dctx.foreground_color),
            None,
            &layout.full_bounds,
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
