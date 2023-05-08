use dui_macros::{multi, multi_from};
use vello::{
    kurbo::{Affine, Insets, Rect, Size},
    peniko::Brush,
};

use crate::{
    defaults::DEFAULT_SPACING,
    drawing::{DrawingContext, LayoutContext},
    layout::{get_id_manger, get_id_manger_mut},
    Alignment, HorizontalAlignment, VerticalALignment,
};

pub trait Element {
    fn body(&self) -> impl Element + View {}

    fn view(&self) -> impl View {
        self.body()
    }

    fn is_leaf(&self) -> bool {
        false
    }
}

impl Element for () {
    fn body(&self) -> impl Element + View {}

    fn is_leaf(&self) -> bool {
        true
    }
}

impl View for () {
    fn layout(&self, _lctx: &mut LayoutContext, _available_rect: Rect) -> Rect {
        Rect::ZERO
    }

    fn draw(&self, _dctx: DrawingContext) {}
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

    fn layout_at(&self, lctx: &mut LayoutContext, available_rect: Rect, _index: usize) -> Rect {
        // Rc::get_mut(&mut lctx.path).unwrap().push(0);
        lctx.path.push(0);

        let layout = self.layout(lctx, available_rect);
        // get_id_manger_mut().insert(path.clone());
        get_id_manger_mut().set_layout_content_rect(Vec::clone(lctx.path), layout);

        // Rc::get_mut(&mut lctx.path).unwrap().pop();
        lctx.path.pop();

        layout
    }

    fn draw_at(&self, dctx: DrawingContext, _index: usize) {
        self.draw(dctx)
    }

    fn is_leaf_at(&self, _index: usize) -> bool {
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
    fn body(&self) -> impl Element + View {}
}

impl<E: ElementIterator> View for VStack<E> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let mut used_rect = available_rect;
        used_rect.y1 = 0.0;

        let mut current_rect = available_rect;
        let mut max_width = 0.0;

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

            if layout.width() > max_width {
                max_width = layout.width()
            }
        }

        used_rect.x1 = used_rect.x0 + max_width;

        // Rc::get_mut(&mut lctx.path).unwrap().pop();
        lctx.path.pop();

        get_id_manger_mut().set_layout_content_rect(Vec::clone(lctx.path), used_rect);

        used_rect
    }

    fn draw(&self, dctx: DrawingContext) {
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

pub struct Rectangle(pub f64, pub f64);

impl Element for Rectangle {}

impl View for Rectangle {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let rect = Rect {
            x0: available_rect.x0,
            y0: available_rect.y0,
            x1: available_rect.x0 + self.0 * lctx.scale_factor,
            y1: available_rect.y0 + self.1 * lctx.scale_factor,
        };

        get_id_manger_mut().set_layout_content_rect(Vec::clone(lctx.path), rect);

        rect
    }

    fn draw(&self, dctx: DrawingContext) {
        let binding = get_id_manger();
        let layout = binding.get_layout(dctx.path.borrow().clone().into());

        dctx.builder.borrow_mut().fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            &dctx.fill_brush,
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
    fn body(&self) -> impl Element + View {}
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

        get_id_manger_mut().set_layout_padding_rect(Vec::clone(lctx.path), used);

        used
    }

    fn draw(&self, dctx: DrawingContext) {
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

        get_id_manger_mut().set_layout_border_rect(Vec::clone(lctx.path), used);

        used
    }

    fn draw(&self, dctx: DrawingContext) {
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

pub struct Fill<V: View> {
    view: V,
    brush: Brush,
}

impl<V: View> Element for Fill<V> {}

impl<V: View> View for Fill<V> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        self.view.layout(lctx, available_rect)
    }

    fn draw(&self, mut dctx: DrawingContext) {
        dctx.fill_brush = self.brush.clone();

        self.view.draw(dctx);
    }
}

pub trait FillImpl<T: View> {
    fn fill(self, brush: impl Into<Brush>) -> Fill<T>;
}

impl<T: View> FillImpl<T> for T {
    fn fill(self, brush: impl Into<Brush>) -> Fill<T> {
        Fill {
            view: self,
            brush: brush.into(),
        }
    }
}

pub struct ExactFrame<V: View> {
    view: V,
    size: Size,
    alignment: Alignment,
}

impl<V: View> ExactFrame<V> {
    pub fn align(self, alignment: Alignment) -> ExactFrame<V> {
        ExactFrame { alignment, ..self }
    }
}

impl<V: View> Element for ExactFrame<V> {}

impl<V: View> View for ExactFrame<V> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        lctx.path.push(0);

        let ref_rect = self.view.layout(lctx, available_rect);
        get_id_manger_mut().remove(lctx.path.clone());

        // Adjust the reference rectangle on the vertical axis
        let ref_rect = match self.alignment.vertical {
            VerticalALignment::Top => ref_rect,
            VerticalALignment::Bottom => Rect {
                y0: available_rect.y0 + self.size.height - ref_rect.height(),
                y1: available_rect.y0 + self.size.height,
                ..ref_rect
            },
            VerticalALignment::Center => Rect {
                y0: available_rect.y0 + self.size.height / 2.0 - ref_rect.height() / 2.0,
                y1: available_rect.y0 + self.size.height / 2.0 + ref_rect.height() / 2.0,
                ..ref_rect
            },
        };

        // Adjust the reference rectangle on the horizontal axis
        let ref_rect = match self.alignment.horizontal {
            HorizontalAlignment::Leading => ref_rect,
            HorizontalAlignment::Trailing => Rect {
                x0: available_rect.x0 + self.size.width - ref_rect.width(),
                x1: available_rect.x0 + self.size.width,
                ..ref_rect
            },
            HorizontalAlignment::Center => Rect {
                x0: available_rect.x0 + self.size.width / 2.0 - ref_rect.width() / 2.0,
                x1: available_rect.x0 + self.size.width / 2.0 + ref_rect.width() / 2.0,
                ..ref_rect
            },
        };

        // Laying out the child again so it's layouts are updated
        self.view.layout(lctx, ref_rect);

        lctx.path.pop();

        let used_rect = Rect {
            x1: available_rect.x0 + self.size.width,
            y1: available_rect.y0 + self.size.height,
            ..available_rect
        };

        get_id_manger_mut().set_layout_content_rect(lctx.path.clone(), used_rect);

        used_rect
    }

    fn draw(&self, dctx: DrawingContext) {
        dctx.path.borrow_mut().push(0);

        self.view.draw(dctx);
    }
}

pub struct LoseFrame<V: View> {
    view: V,
    min_size: Size,
    max_size: Size,
    alignment: Alignment,
}

impl<V: View> LoseFrame<V> {
    pub fn align(self, alignment: Alignment) -> LoseFrame<V> {
        LoseFrame { alignment, ..self }
    }
}

impl<V: View> Element for LoseFrame<V> {}

impl<V: View> View for LoseFrame<V> {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let size = available_rect.size().clamp(self.min_size, self.max_size);

        lctx.path.push(0);

        let ref_rect = self.view.layout(lctx, available_rect);

        // Adjust the reference rectangle on the vertical axis
        let ref_rect = match self.alignment.vertical {
            VerticalALignment::Top => ref_rect,
            VerticalALignment::Bottom => Rect {
                y0: available_rect.y0 + size.height - ref_rect.height(),
                y1: available_rect.y0 + size.height,
                ..ref_rect
            },
            VerticalALignment::Center => Rect {
                y0: available_rect.y0 + size.height / 2.0 - ref_rect.height() / 2.0,
                y1: available_rect.y0 + size.height / 2.0 + ref_rect.height() / 2.0,
                ..ref_rect
            },
        };

        // Adjust the reference rectangle on the horizontal axis
        let ref_rect = match self.alignment.horizontal {
            HorizontalAlignment::Leading => ref_rect,
            HorizontalAlignment::Trailing => Rect {
                x0: available_rect.x0 + size.width - ref_rect.width(),
                x1: available_rect.x0 + size.width,
                ..ref_rect
            },
            HorizontalAlignment::Center => Rect {
                x0: available_rect.x0 + size.width / 2.0 - ref_rect.width() / 2.0,
                x1: available_rect.x0 + size.width / 2.0 + ref_rect.width() / 2.0,
                ..ref_rect
            },
        };

        // Laying out the child again so it's layouts are updated
        get_id_manger_mut().remove(lctx.path.clone());
        self.view.layout(lctx, ref_rect);

        lctx.path.pop();

        let used_rect = Rect {
            x1: available_rect.x0 + size.width,
            y1: available_rect.y0 + size.height,
            ..available_rect
        };

        get_id_manger_mut().set_layout_content_rect(lctx.path.clone(), used_rect);

        used_rect
    }

    fn draw(&self, dctx: DrawingContext) {
        dctx.path.borrow_mut().push(0);

        self.view.draw(dctx);
    }
}

pub trait FrameImpl<T: View> {
    fn frame(self, size: impl Into<Size>) -> ExactFrame<T>;
    fn frame_min_max(self, min: impl Into<Size>, max: impl Into<Size>) -> LoseFrame<T>;
}

impl<T: View> FrameImpl<T> for T {
    fn frame(self, size: impl Into<Size>) -> ExactFrame<T> {
        ExactFrame {
            view: self,
            size: size.into(),
            alignment: Alignment::default(),
        }
    }

    fn frame_min_max(self, min: impl Into<Size>, max: impl Into<Size>) -> LoseFrame<T> {
        LoseFrame {
            view: self,
            min_size: min.into(),
            max_size: max.into(),
            alignment: Alignment::default(),
        }
    }
}

pub struct Text(String);

impl Text {
    pub fn new(text: impl Into<String>) -> Text {
        Text(text.into())
    }
}

impl Element for Text {}

impl View for Text {
    fn layout(&self, lctx: &mut LayoutContext, available_rect: Rect) -> Rect {
        let rect = lctx.font_manager.borrow().layout(
            None,
            20.0,
            lctx.scale_factor as _,
            available_rect,
            &self.0,
        );

        get_id_manger_mut().set_layout_content_rect(lctx.path.clone(), rect);

        rect
    }

    fn draw(&self, dctx: DrawingContext) {
        let binding = get_id_manger();
        let rect = binding.get_layout(dctx.path.borrow_mut().clone().into());

        dctx.font_manager.borrow_mut().add(
            &mut dctx.builder.borrow_mut(),
            None,
            20.0,
            dctx.scale_factor as _,
            Some(&dctx.foreground_color),
            Affine::IDENTITY,
            &self.0,
            rect.content_bounds,
        );
    }
}
