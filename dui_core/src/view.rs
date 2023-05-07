use std::rc::Rc;

use dui_macros::{multi, multi_from};
use vello::{
    kurbo::{Affine, Insets, Rect},
    peniko::Brush,
};

use crate::{
    defaults::DEFAULT_SPACING,
    drawing::{DrawingContext, LayoutContext},
    layout::{get_id_manger, get_id_manger_mut},
};

pub trait Element {
    fn body(&self) -> impl Element + View {
        ()
    }

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

    fn draw(&self, dctx: DrawingContext) {}
}

pub trait ElementIterator {
    fn len(&self) -> usize;
    fn layout_at(&self, lctx: &mut LayoutContext, available_rect: Rect, index: usize) -> Rect;
    fn draw_at(&self, dctx: DrawingContext, index: usize);
    fn is_leaf_at(&self, index: usize) -> bool;
}

impl<V: View> ElementIterator for V {
    fn len(&self) -> usize {
        1
    }

    fn layout_at(&self, lctx: &mut LayoutContext, available_rect: Rect, index: usize) -> Rect {
        // Rc::get_mut(&mut lctx.path).unwrap().push(0);
        lctx.path.push(0);

        let layout = self.layout(lctx, available_rect);
        // get_id_manger_mut().insert(path.clone());
        get_id_manger_mut().set_layout_content_rect(Vec::clone(&lctx.path).into(), layout);

        // Rc::get_mut(&mut lctx.path).unwrap().pop();
        lctx.path.pop();

        layout
    }

    fn draw_at(&self, dctx: DrawingContext, index: usize) {
        self.draw(dctx)
    }

    fn is_leaf_at(&self, index: usize) -> bool {
        false
    }
}

pub trait View: Element {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        self.body().layout(lctx, available_rect)
    }

    fn draw(&self, dctx: DrawingContext) {
        self.body().draw(dctx);
    }
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

        get_id_manger_mut().set_layout_content_rect(Vec::clone(&lctx.path).into(), used_rect);

        used_rect
    }

    fn draw(&self, mut dctx: DrawingContext) {
        // Rc::get_mut(&mut dctx.path).unwrap().push(0);
        dctx.path.borrow_mut().push(0);

        for i in 0..self.element.len() {
            // *Rc::get_mut(&mut dctx.path).unwrap().last_mut().unwrap() = i as u32;
            *dctx.path.borrow_mut().last_mut().unwrap() = i as u32;

            self.element.draw_at(dctx.clone(), i)
        }

        // Rc::get_mut(&mut dctx.path).unwrap().pop();
        dctx.path.borrow_mut().pop();
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

        get_id_manger_mut().set_layout_content_rect(Vec::clone(&lctx.path).into(), rect);

        rect
    }

    fn draw(&self, mut dctx: DrawingContext) {
        let binding = get_id_manger();
        let layout = binding.get_layout(dctx.path.borrow().clone().into());

        dctx.builder.borrow_mut().fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            &dctx.foreground_color,
            None,
            &layout.content_bounds,
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

pub struct Padding<E: View> {
    element: E,
    edges: Insets,
}

impl<E: View> Element for Padding<E> {
    fn body(&self) -> impl Element + View {
        ()
    }
}

impl<E: View> View for Padding<E> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let edges = Insets {
            x0: self.edges.x0 * lctx.scale_factor,
            y0: self.edges.y0 * lctx.scale_factor,
            x1: self.edges.x1 * lctx.scale_factor,
            y1: self.edges.y1 * lctx.scale_factor,
        };

        let new_rect = Rect {
            x0: available_rect.x0 + edges.x0,
            y0: available_rect.y0 + edges.y0,
            x1: available_rect.x1 - edges.x1,
            y1: available_rect.y1 - edges.y1,
        };

        let layout = self.element.layout(lctx, new_rect);

        let used = Rect {
            x0: layout.x0 - edges.x0,
            y0: layout.y0 - edges.y0,
            x1: layout.x1 + edges.x1,
            y1: layout.y1 + edges.y1,
        };

        get_id_manger_mut().set_layout_padding_rect(Vec::clone(&lctx.path).into(), used);

        used
    }

    fn draw(&self, mut dctx: DrawingContext) {
        let binding = get_id_manger();
        let layout = binding.get_layout(dctx.path.borrow().clone().into());

        dctx.builder.borrow_mut().fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            &dctx.background_brush,
            None,
            &layout.padding_bounds,
        );

        self.element.draw(dctx);
    }
}

pub trait PaddingImpl<T: View> {
    fn padding(self, edges: impl Into<Insets>) -> Padding<T>;
}

impl<T: View> PaddingImpl<T> for T {
    fn padding(self, edges: impl Into<Insets>) -> Padding<T> {
        Padding {
            element: self,
            edges: edges.into(),
        }
    }
}

pub struct Border<E: View> {
    element: E,

    brush: Brush,
    edges: Insets,
}

impl<E: View> Element for Border<E> {
    fn body(&self) -> impl Element + View {}
}

impl<E: View> View for Border<E> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let edges = Insets {
            x0: self.edges.x0 * lctx.scale_factor,
            y0: self.edges.y0 * lctx.scale_factor,
            x1: self.edges.x1 * lctx.scale_factor,
            y1: self.edges.y1 * lctx.scale_factor,
        };

        let new_rect = Rect {
            x0: available_rect.x0 + edges.x0,
            y0: available_rect.y0 + edges.y0,
            x1: available_rect.x1 - edges.x1,
            y1: available_rect.y1 - edges.y1,
        };

        let layout = self.element.layout(lctx, new_rect);

        let used = Rect {
            x0: layout.x0 - edges.x0,
            y0: layout.y0 - edges.y0,
            x1: layout.x1 + edges.x1,
            y1: layout.y1 + edges.y1,
        };

        get_id_manger_mut().set_layout_border_rect(Vec::clone(&lctx.path).into(), used);

        used
    }

    fn draw(&self, mut dctx: DrawingContext) {
        let binding = get_id_manger();
        let layout = binding.get_layout(Vec::clone(&dctx.path.borrow()).into());

        dctx.builder.borrow_mut().fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            // &Brush::Solid(dctx.foreground_color),
            &self.brush,
            None,
            &layout.border_bounds,
        );

        self.element.draw(dctx);
    }
}

pub trait BorderImpl<T: View> {
    fn border(self, edges: impl Into<Insets>, brush: impl Into<Brush>) -> Border<T>;
}

impl<T: View> BorderImpl<T> for T {
    fn border(self, edges: impl Into<Insets>, brush: impl Into<Brush>) -> Border<T> {
        Border {
            element: self,
            brush: brush.into(),
            edges: edges.into(),
        }
    }
}

pub struct Background<V: View> {
    view: V,
    brush: Brush,
}

impl<V: View> Element for Background<V> {}

impl<V: View> View for Background<V> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        self.view.layout(lctx, available_rect)
    }

    fn draw(&self, mut dctx: DrawingContext) {
        dctx.background_brush = self.brush.clone();

        self.view.draw(dctx);
    }
}

pub trait BackgroundImpl<T: View> {
    fn background(self, brush: impl Into<Brush>) -> Background<T>;
}

impl<T: View> BackgroundImpl<T> for T {
    fn background(self, brush: impl Into<Brush>) -> Background<T> {
        Background {
            view: self,
            brush: brush.into(),
        }
    }
}
