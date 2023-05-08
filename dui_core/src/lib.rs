#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_const_equality)]

pub mod defaults;
pub mod drawing;
pub mod layout;
pub mod view;
pub mod simple_text;

#[derive(Clone, Copy)]
pub struct Alignment {
    pub(crate) horizontal: HorizontalAlignment,
    pub(crate) vertical: VerticalALignment,
}

impl Default for Alignment {
    fn default() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalALignment::Center,
        }
    }
}

impl Alignment {
    pub const LEADING: Alignment = Alignment {
        horizontal: HorizontalAlignment::Leading,
        vertical: VerticalALignment::Center,
    };

    pub const TRAILING: Alignment = Alignment {
        horizontal: HorizontalAlignment::Trailing,
        vertical: VerticalALignment::Center,
    };

    pub const CENTER: Alignment = Alignment {
        horizontal: HorizontalAlignment::Center,
        vertical: VerticalALignment::Center,
    };

    pub const TOP: Alignment = Alignment {
        horizontal: HorizontalAlignment::Center,
        vertical: VerticalALignment::Top,
    };

    pub const BOTTOM: Alignment = Alignment {
        horizontal: HorizontalAlignment::Center,
        vertical: VerticalALignment::Bottom,
    };

    pub const TOP_LEADING: Alignment = Alignment {
        horizontal: HorizontalAlignment::Leading,
        vertical: VerticalALignment::Top,
    };

    pub const TOP_TRAILING: Alignment = Alignment {
        horizontal: HorizontalAlignment::Trailing,
        vertical: VerticalALignment::Top,
    };

    pub const BOTTOM_LEADING: Alignment = Alignment {
        horizontal: HorizontalAlignment::Leading,
        vertical: VerticalALignment::Bottom,
    };

    pub const BOTTOM_TRAILING: Alignment = Alignment {
        horizontal: HorizontalAlignment::Trailing,
        vertical: VerticalALignment::Bottom,
    };
}

#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Leading,
    Trailing,
    Center,
}

#[derive(Clone, Copy)]
pub enum VerticalALignment {
    Top,
    Bottom,
    Center,
}
