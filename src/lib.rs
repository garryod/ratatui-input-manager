#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Ratatui widgets displaying keybinds
#[cfg(feature = "widgets")]
pub mod widgets;

use crossterm::event::{Event, KeyCode};
pub use ratatui_input_manager_derive::keymap;

/// Key binding metadata, including the handlded key presses and a description of behaviour
pub struct KeyBind {
    /// The keys which is handled by this binding
    pub keys: &'static [KeyCode],
    /// A brief description of the expected behaviour
    pub description: Option<&'static str>,
}

/// A mapping between keybinds and methods on the type which mutate the state
pub trait KeyMap {
    /// Metadata about the key presses which are handled
    const KEYBINDS: &'static [KeyBind];

    /// Handle a [`crossterm::event::Event`] by calling the appropriate handler method
    fn handle(&mut self, event: &Event);
}

/// A dyn compatible equivilent to [`KeyMap`]
#[expect(missing_docs)]
pub trait DynKeyMap {
    fn keybinds(&self) -> &'static [KeyBind];

    fn handle(&mut self, event: &Event);
}

impl<T: KeyMap> DynKeyMap for T {
    fn keybinds(&self) -> &'static [KeyBind] {
        T::KEYBINDS
    }

    fn handle(&mut self, event: &Event) {
        self.handle(event)
    }
}
