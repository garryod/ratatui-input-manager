use crate::{KeyBind, VimlikeExt};
use itertools::Itertools;
use ratatui_core::{
    buffer::Buffer,
    layout::Constraint,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use ratatui_widgets::{
    block::Block,
    table::{Cell, Row, Table},
};

/// A [`Widget`] displaying a table of bound keys and their description
pub struct Help<'k, E: 'static> {
    keybinds: &'k [KeyBind<E>],
    block: Option<Block<'k>>,
    key_style: Style,
    description_style: Style,
}

impl<'k, E: 'static> Help<'k, E> {
    /// Construct a [`Help`] [`Widget`] from a collection of [`KeyBind`]s, typically obtained by
    /// inspecting the metadata as [`crate::KeyMap::KEYBINDS`] or
    /// [`crate::DynKeyMap::keybinds`]
    pub fn new(keybinds: &'k [KeyBind<E>]) -> Self {
        Self {
            keybinds,
            block: None,
            key_style: Style::default(),
            description_style: Style::default(),
        }
    }

    /// Wraps the help table with the given [`Block`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'k>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the style of the key code text
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    /// Sets the style of the description text
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn description_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }
}

impl<'k, E: 'static + VimlikeExt<'k>> Widget for Help<'k, E> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let table = Table::new(
            self.keybinds.iter().map(|KeyBind { keys, description }| {
                Row::new([
                    Cell::new(
                        keys.iter()
                            .map(|key| key.as_vimlike())
                            .format(", ")
                            .to_string(),
                    )
                    .style(self.key_style),
                    Cell::new(description.unwrap_or_default()).style(self.description_style),
                ])
            }),
            [Constraint::Min(8), Constraint::Fill(1)],
        );
        let area = match self.block {
            Some(block) => {
                block.clone().render(area, buf);
                block.inner(area)
            }
            None => area,
        };
        table.render(area, buf);
    }
}

/// A [`Widget`] displaying a single row of bound keys and their descriptions
pub struct HelpBar<'k, E: 'static> {
    keybinds: &'k [KeyBind<E>],
    key_style: Style,
    description_style: Style,
    separator_style: Style,
}

impl<'k, E: 'static> HelpBar<'k, E> {
    /// Construct a [`HelpBar`] [`Widget`] from a collection of [`KeyBind`]s, typically obtained by
    /// inspecting the metadata as [`crate::KeyMap::KEYBINDS`] or
    /// [`crate::DynKeyMap::keybinds`]
    pub fn new(keybinds: &'k [KeyBind<E>]) -> Self {
        Self {
            keybinds,
            key_style: Style::default(),
            description_style: Style::default(),
            separator_style: Style::default(),
        }
    }

    /// Sets the style of the key code text
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    /// Sets the style of the description text
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn description_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }

    /// Sets the style of the separators used to distinguish key bindings
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn separator_style(mut self, style: Style) -> Self {
        self.separator_style = style;
        self
    }
}

impl<'k, E: 'static + VimlikeExt<'k>> Widget for HelpBar<'k, E> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from_iter(self.keybinds.iter().enumerate().flat_map(
            |(idx, KeyBind { keys, description })| {
                [
                    Span::styled(if idx == 0 { "" } else { " | " }, self.separator_style),
                    Span::styled(
                        format!("{}: ", description.unwrap_or_default()),
                        self.description_style,
                    ),
                    Span::styled(
                        keys.iter()
                            .map(|key| key.as_vimlike())
                            .format(", ")
                            .to_string(),
                        self.key_style,
                    ),
                ]
            },
        ))
        .render(area, buf);
    }
}
