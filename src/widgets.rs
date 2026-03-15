use crate::KeyBind;
use ratatui_core::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::Widget,
};
use ratatui_widgets::table::{Cell, Row, Table};

/// A widget displaying a table of bound keys and their description
pub struct Help<'k> {
    keybinds: &'k [KeyBind],
}

impl<'k> Help<'k> {
    /// Construct a [`Help`] [`Widget`] from a collection of [`KeyBind`]s, typically obtained by
    /// inspecting the metadata as [`crate::KeyMap::KEYBINDS`] or
    /// [`crate::DynKeyMap::keybinds`]
    pub fn new(keybinds: &'k [KeyBind]) -> Self {
        Self { keybinds }
    }
}

impl<'k> Widget for Help<'k> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let rows = self.keybinds.iter().map(|KeyBind { key, description }| {
            Row::new([
                Cell::new(format!("{}", key)),
                Cell::new(description.unwrap_or_default()),
            ])
        });
        Table::new(rows, [Constraint::Min(8), Constraint::Fill(1)]).render(area, buf);
    }
}
