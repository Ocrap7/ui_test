use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use vello::kurbo::Rect;

lazy_static::lazy_static! {
    static ref ID_MANAGER: RwLock<IdManager> = RwLock::new(IdManager {
        id_mappings: HashMap::new(),
    });
}

pub fn get_id_manger() -> RwLockReadGuard<'static, IdManager> {
    ID_MANAGER.read().unwrap()
}

pub fn get_id_manger_mut() -> RwLockWriteGuard<'static, IdManager> {
    ID_MANAGER.write().unwrap()
}

/// This represents the viewable layout of an element
///
/// `border_bounds` represents the total physical space
/// `padding_bounds` is the region taken up by the padding and content
/// `content_bounds` is the region taken up by only the content
///
/// These are all relative to the screen. If there is no border or padding, they should be equal to the content bounds
#[derive(Debug, Default)]
pub struct Layout {
    pub border_bounds: Rect,
    pub padding_bounds: Rect,
    pub content_bounds: Rect,
}

pub const LAYOUT_ZERO: Layout = Layout {
    border_bounds: Rect::ZERO,
    padding_bounds: Rect::ZERO,
    content_bounds: Rect::ZERO,
};

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Id(Vec<u32>);

impl From<Vec<u32>> for Id {
    fn from(value: Vec<u32>) -> Self {
        Id(value)
    }
}

#[derive(Debug)]
pub struct IdManager {
    pub(crate) id_mappings: HashMap<Id, Layout>,
}

impl IdManager {
    pub fn insert(&mut self, id: impl Into<Id>) {
        self.id_mappings.insert(id.into(), Layout::default());
    }

    pub fn set_layout_padding_rect(&mut self, id: Id, layout: Rect) -> Option<Layout> {
        if let Some(full) = self.id_mappings.get_mut(&id) {
            full.padding_bounds = layout;
            None
        } else {
            self.id_mappings.insert(
                id,
                Layout {
                    padding_bounds: layout,
                    border_bounds: layout,
                    content_bounds: layout,
                },
            )
        }
    }

    pub fn set_layout_border_rect(&mut self, id: Id, layout: Rect) -> Option<Layout> {
        if let Some(full) = self.id_mappings.get_mut(&id) {
            full.border_bounds = layout;
            None
        } else {
            self.id_mappings.insert(
                id,
                Layout {
                    padding_bounds: layout,
                    border_bounds: layout,
                    content_bounds: layout,
                },
            )
        }
    }

    pub fn set_layout_content_rect(&mut self, id: Id, layout: Rect) -> Option<Layout> {
        if let Some(full) = self.id_mappings.get_mut(&id) {
            full.content_bounds = layout;
            None
        } else {
            self.id_mappings.insert(
                id,
                Layout {
                    padding_bounds: layout,
                    border_bounds: layout,
                    content_bounds: layout,
                },
            )
        }
    }

    pub fn get_layout(&self, id: Id) -> &Layout {
        self.id_mappings.get(&id).unwrap_or(&LAYOUT_ZERO)
    }
}
