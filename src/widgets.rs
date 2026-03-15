use crate::KeyBind;
use itertools::Itertools;
use ratatui_core::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Line,
    widgets::Widget,
};
use ratatui_widgets::table::{Cell, Row, Table};

/// A [`Widget`] displaying a table of bound keys and their description
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

/// A [`Widget`] displaying a single row of bound keys and their descriptions
pub struct HelpBar<'k> {
    keybinds: &'k [KeyBind],
}

impl<'k> HelpBar<'k> {
    /// Construct a [`HelpBar`] [`Widget`] from a collection of [`KeyBind`]s, typically obtained by
    /// inspecting the metadata as [`crate::KeyMap::KEYBINDS`] or
    /// [`crate::DynKeyMap::keybinds`]
    pub fn new(keybinds: &'k [KeyBind]) -> Self {
        Self { keybinds }
    }
}

impl<'k> Widget for HelpBar<'k> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        #![allow(unstable_name_collisions)]
        Line::from_iter(
            self.keybinds
                .iter()
                .map(|KeyBind { key, description }| {
                    format!("{}: {}", description.unwrap_or_default(), key)
                })
                .intersperse(" | ".to_string()),
        )
        .render(area, buf);
    }
}
