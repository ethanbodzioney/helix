//! `widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or both.
//!
//! All widgets are implemented using the builder pattern and are consumable objects. They are not
//! meant to be stored but used as *commands* to draw common figures in the UI.
//!
//! The available widgets are:
//! - [`Block`]
// //! - [`List`]
// //! - [`Table`]
//! - [`Paragraph`]

mod block;
// mod list;
mod paragraph;
mod reflow;
mod table;

pub use self::block::{Block, BorderType};
// pub use self::list::{List, ListItem, ListState};
pub use self::paragraph::{Paragraph, Wrap};
pub use self::table::{Cell, Row, Table, TableState};

use crate::{buffer::Buffer, layout::Rect};
use bitflags::bitflags;

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
    pub struct Borders: u32 {
        /// Show no border (default)
        const NONE  = 0b0000_0001;
        /// Show the top border
        const TOP   = 0b0000_0010;
        /// Show the right border
        const RIGHT = 0b0000_0100;
        /// Show the bottom border
        const BOTTOM = 0b000_1000;
        /// Show the left border
        const LEFT = 0b0001_0000;
        /// Show all borders
        const ALL = Self::TOP.bits | Self::RIGHT.bits | Self::BOTTOM.bits | Self::LEFT.bits;
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn render(self, area: Rect, buf: &mut Buffer);
}
