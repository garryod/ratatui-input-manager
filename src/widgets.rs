use crate::{Backend, KeyBind, KeyPress};
use itertools::Itertools;
use ratatui_core::{
    buffer::Buffer,
    layout::{Constraint, HorizontalAlignment, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::Widget,
};
use ratatui_widgets::{
    block::Block,
    table::{Cell, Row, Table},
};

/// A [`ratatui_core::widgets::Widget`] displaying a table of bound keys and
/// their description
///
/// # Example
///
/// ```rust
/// use crossterm::event::KeyCode;
/// use ratatui_core::{
///     style::{Color, Style},
///     terminal::Frame,
/// };
/// use ratatui_input_manager::{CrosstermBackend, KeyMap, keymap};
/// use ratatui_input_manager::widgets::Help;
/// use ratatui_widgets::block::Block;
///
/// #[derive(Default)]
/// struct App;
///
/// #[keymap(backend = "crossterm")]
/// impl App {
///     #[keybind(pressed(key = KeyCode::Char('q')))]
///     fn quit(&mut self) {}
/// }
///
/// fn render_help(frame: &mut Frame) {
///     let help = Help::<CrosstermBackend>::new(App::KEYBINDS)
///         .block(Block::default().title("Controls"))
///         .key_style(Style::default().fg(Color::Cyan));
///     frame.render_widget(help, frame.area());
/// }
/// ```
pub struct Help<'k, B: Backend + 'static> {
    keybinds: &'k [KeyBind<B>],
    block: Option<Block<'k>>,
    style: Style,
    key_style: Style,
    description_style: Style,
}

impl<'k, B: Backend> Help<'k, B> {
    /// Construct a [`Help`] [`Widget`] from a collection of [`KeyBind`]s, typically obtained by
    /// inspecting the metadata as [`crate::KeyMap::KEYBINDS`] or
    /// [`crate::DynKeyMap::keybinds`]
    pub fn new(keybinds: &'k [KeyBind<B>]) -> Self {
        Self {
            keybinds,
            block: None,
            style: Style::default(),
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

    /// Sets the global style of the help table
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style of the key code text
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    /// Sets the style of the description text
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn description_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }
}

impl<'k, B: Backend + 'static> Widget for Help<'k, B>
where
    KeyPress<B>: std::fmt::Display,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let max_key_width = self
            .keybinds
            .iter()
            .map(|kb| {
                let key_lens: Vec<usize> = kb.pressed.iter().map(|p| p.to_string().len()).collect();
                key_lens.iter().sum::<usize>() + key_lens.len().saturating_sub(1) * 2
            })
            .max()
            .unwrap_or_default()
            .max(3) as u16;

        let table = Table::new(
            self.keybinds.iter().map(
                |KeyBind {
                     pressed,
                     description,
                     ..
                 }| {
                    let key_str = pressed
                        .iter()
                        .map(|key| key.to_string())
                        .format(", ")
                        .to_string();
                    let mut text = Text::from(key_str);
                    text.alignment = Some(HorizontalAlignment::Right);
                    Row::new([
                        Cell::new(text).style(self.key_style),
                        Cell::new(description.unwrap_or_default()).style(self.description_style),
                    ])
                },
            ),
            [Constraint::Length(max_key_width), Constraint::Fill(1)],
        )
        .style(self.style);
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
///
/// # Example
///
/// ```rust
/// use crossterm::event::KeyCode;
/// use ratatui_core::{
///     style::{Color, Style},
///     terminal::Frame,
/// };
/// use ratatui_input_manager::{CrosstermBackend, KeyMap, keymap};
/// use ratatui_input_manager::widgets::HelpBar;
///
/// #[derive(Default)]
/// struct App;
///
/// #[keymap(backend = "crossterm")]
/// impl App {
///     #[keybind(pressed(key = KeyCode::Char('q')))]
///     fn quit(&mut self) {}
/// }
///
/// fn render_help_bar(frame: &mut Frame) {
///     let help = HelpBar::<CrosstermBackend>::new(App::KEYBINDS)
///         .key_style(Style::default().fg(Color::Cyan))
///         .separator_style(Style::default().fg(Color::DarkGray));
///     frame.render_widget(help, frame.area());
/// }
/// ```
pub struct HelpBar<'k, B: Backend + 'static> {
    keybinds: &'k [KeyBind<B>],
    style: Style,
    key_style: Style,
    description_style: Style,
    separator_style: Style,
}

impl<'k, B: Backend> HelpBar<'k, B> {
    /// Construct a [`HelpBar`] [`Widget`] from a collection of [`KeyBind`]s, typically obtained by
    /// inspecting the metadata as [`crate::KeyMap::KEYBINDS`] or
    /// [`crate::DynKeyMap::keybinds`]
    pub fn new(keybinds: &'k [KeyBind<B>]) -> Self {
        Self {
            keybinds,
            style: Style::default(),
            key_style: Style::default(),
            description_style: Style::default(),
            separator_style: Style::default(),
        }
    }

    /// Sets the global style of the help bar
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style of the key code text
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    /// Sets the style of the description text
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn description_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }

    /// Sets the style of the separators used to distinguish key bindings
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn separator_style(mut self, style: Style) -> Self {
        self.separator_style = style;
        self
    }
}

impl<'k, B: Backend + 'static> Widget for HelpBar<'k, B>
where
    KeyPress<B>: std::fmt::Display,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from_iter(self.keybinds.iter().enumerate().flat_map(
            |(
                idx,
                KeyBind {
                    pressed,
                    description,
                    ..
                },
            )| {
                [
                    Span::styled(if idx == 0 { "" } else { " | " }, self.separator_style),
                    Span::styled(
                        format!("{}: ", description.unwrap_or_default()),
                        self.description_style,
                    ),
                    Span::styled(
                        pressed
                            .iter()
                            .map(|press| press.to_string())
                            .format(", ")
                            .to_string(),
                        self.key_style,
                    ),
                ]
            },
        ))
        .style(self.style)
        .render(area, buf);
    }
}
