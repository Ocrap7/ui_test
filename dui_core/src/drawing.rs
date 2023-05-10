use dui_util::Rf;
use vello::{kurbo::Rect, peniko::Brush, SceneBuilder};

use crate::{layout::Id, simple_text::FontManager};

pub struct PathPusher(Rf<Vec<u32>>);

impl PathPusher {
    pub fn new(dctx: &DrawingContext) -> PathPusher {
        PathPusher(dctx.path.clone())
    }

    pub fn set_last(&self, index: u32) {
        *self.0.borrow_mut().last_mut().unwrap() = index;
    }
}

impl Drop for PathPusher {
    fn drop(&mut self) {
        self.0.borrow_mut().pop();
    }
}

#[derive(Clone)]
pub struct DrawingContext<'a> {
    pub builder: Rf<SceneBuilder<'a>>,
    pub path: Rf<Vec<u32>>,

    pub font_manager: Rf<FontManager>,

    pub background_brush: Brush,
    pub fill_brush: Brush,
    pub foreground_color: Brush,

    pub bounding: Rect,
    pub first: bool,

    pub scale_factor: f64,
}

impl DrawingContext<'_> {
    pub fn push(&self) -> PathPusher {
        self.path.borrow_mut().push(0);
        PathPusher::new(self)
    }

    pub fn id(&self) -> Id {
        self.path.borrow().clone().into()
    }
}

pub struct LayoutPathPusher<'a>(&'a mut Vec<u32>);

impl <'a> LayoutPathPusher<'a> {
    pub fn new(dctx: &'a mut LayoutContext) -> LayoutPathPusher<'a> {
        LayoutPathPusher(dctx.path)
    }

    pub fn set_last(&mut self, index: u32) {
        *self.0.last_mut().unwrap() = index;
    }
}

impl Drop for LayoutPathPusher<'_> {
    fn drop(&mut self) {
        self.0.pop();
    }
}

#[derive(Debug)]
pub struct LayoutContext<'a> {
    pub font_manager: Rf<FontManager>,
    // pub path: Rc<Vec<u32>>,
    pub path: &'a mut Vec<u32>,
    pub scale_factor: f64,
}

impl LayoutContext<'_> {
    pub fn push(&mut self) -> LayoutPathPusher {
        LayoutPathPusher::new(self)
    }

    pub fn id(&self) -> Id {
        self.path.clone().into()
    }
}
