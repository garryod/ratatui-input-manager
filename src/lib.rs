#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crossterm::event::Event;
pub use ratatui_input_manager_derive::keymap;

/// A mapping between keybinds and methods on the type which mutate the state
pub trait KeyMap {
    /// Handle a [`crossterm::event::Event`] by calling the appropriate handler method
    fn handle(&mut self, event: &Event);
}
