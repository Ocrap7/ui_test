use std::{collections::HashMap, sync::RwLock};

use vello::kurbo::Rect;

lazy_static::lazy_static! {
    static ref ID_MANAGER: RwLock<IdManager> = RwLock::new(IdManager {
        id_mappings: HashMap::new(),
        next_id: Id(rand::random())
    });
}

#[derive(Debug, Default)]
pub struct Layout {
    full_bounds: Rect,
    border_bounds: Rect,
    content_bounds: Rect,
}

pub const LAYOUT_ZERO: Layout = Layout {
    full_bounds: Rect::ZERO,
    border_bounds: Rect::ZERO,
    content_bounds: Rect::ZERO,
};

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Id(u64);

#[derive(Debug)]
pub struct IdManager {
    pub(crate) id_mappings: HashMap<Id, Layout>,
    next_id: Id,
}

impl IdManager {
    pub fn gen_id(&mut self) -> Id {
        self.next_id.0 += 1;
        Id(self.next_id.0 - 1)
    }

    pub fn gen_insert_zero(&mut self) -> Id {
        let id = self.gen_id();
        self.id_mappings.insert(id, Default::default());
        id
    }

    pub fn set_layout_full_rect(&mut self, id: Id, layout: Rect) -> Option<Layout> {
        if let Some(full) = self.id_mappings.get_mut(&id) {
            full.full_bounds = layout;
            None
        } else {
            self.id_mappings.insert(
                id,
                Layout {
                    full_bounds: layout,
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
                    full_bounds: layout,
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
                    full_bounds: layout,
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
