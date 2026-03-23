#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#[cfg(feature = "crossterm")]
use std::fmt::Display;

/// Ratatui widgets displaying keybinds
#[cfg(feature = "widgets")]
pub mod widgets;
pub use ratatui_input_manager_derive::keymap;

/// Key binding metadata, including the handlded key presses and a description of behaviour
pub struct KeyBind<K: 'static> {
    /// The keys which is handled by this binding
    pub keys: &'static [K],
    /// A brief description of the expected behaviour
    pub description: Option<&'static str>,
}

/// A mapping between keybinds and methods on the type which mutate the state
pub trait KeyMap<K: 'static, E> {
    /// Metadata about the key presses which are handled
    const KEYBINDS: &'static [KeyBind<K>];

    /// Handle a [`crossterm::event::Event`] by calling the appropriate handler method
    fn handle(&mut self, event: &E);
}

/// A dyn compatible equivilent to [`KeyMap`]
#[expect(missing_docs)]
pub trait DynKeyMap<K: 'static, E> {
    fn keybinds(&self) -> &'static [KeyBind<K>];

    fn handle(&mut self, event: &E);
}

impl<K: 'static, E, T: KeyMap<K, E>> DynKeyMap<K, E> for T {
    fn keybinds(&self) -> &'static [KeyBind<K>] {
        T::KEYBINDS
    }

    fn handle(&mut self, event: &E) {
        self.handle(event)
    }
}

struct Vimlike<'k, K>(&'k K);

/// Provides a methanism for [`Display`]ing key press like types in a manner similar to key press
/// hints in vim.
pub trait VimlikeExt<'k>: Sized {
    /// Convert this type to one which implements [`Display`] in a manner similar to key press
    /// hints in vim.
    fn as_vimlike(&'k self) -> impl Display + 'k;
}

impl<'k, K: 'k> VimlikeExt<'k> for K
where
    Vimlike<'k, K>: Display,
{
    fn as_vimlike(&'k self) -> impl Display {
        Vimlike(self)
    }
}

#[cfg(feature = "crossterm")]
impl<'k> Display for Vimlike<'k, crossterm::event::KeyCode> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            crossterm::event::KeyCode::Backspace => write!(f, "<BS>"),
            crossterm::event::KeyCode::Enter => write!(f, "<CR>"),
            crossterm::event::KeyCode::Left => write!(f, "<Left>"),
            crossterm::event::KeyCode::Right => write!(f, "<Right>"),
            crossterm::event::KeyCode::Up => write!(f, "<Up>"),
            crossterm::event::KeyCode::Down => write!(f, "<Down>"),
            crossterm::event::KeyCode::Home => write!(f, "<Home>"),
            crossterm::event::KeyCode::End => write!(f, "<End>"),
            crossterm::event::KeyCode::PageUp => write!(f, "<PageUp>"),
            crossterm::event::KeyCode::PageDown => write!(f, "<PageDown>"),
            crossterm::event::KeyCode::Tab => write!(f, "<Tab>"),
            crossterm::event::KeyCode::BackTab => write!(f, "<S-Tab>"),
            crossterm::event::KeyCode::Delete => write!(f, "<Del>"),
            crossterm::event::KeyCode::Insert => write!(f, "<Insert>"),
            crossterm::event::KeyCode::F(num) => write!(f, "<F{num}>"),
            crossterm::event::KeyCode::Char(c) => write!(f, "{c}"),
            crossterm::event::KeyCode::Null => write!(f, "<Nul>"),
            crossterm::event::KeyCode::Esc => write!(f, "<Esc>"),
            crossterm::event::KeyCode::CapsLock => write!(f, "<CapsLock>"),
            crossterm::event::KeyCode::NumLock => write!(f, "<NumLock>"),
            crossterm::event::KeyCode::ScrollLock => write!(f, "<ScrollLock>"),
            crossterm::event::KeyCode::PrintScreen => write!(f, "<PrintScreen>"),
            crossterm::event::KeyCode::Pause => write!(f, "<Pause>"),
            crossterm::event::KeyCode::Menu => write!(f, "<Menu>"),
            crossterm::event::KeyCode::KeypadBegin => write!(f, "<kBegin>"),
            crossterm::event::KeyCode::Media(code) => match code {
                crossterm::event::MediaKeyCode::Play => write!(f, "<MediaPlay>"),
                crossterm::event::MediaKeyCode::Pause => write!(f, "<MediaPause>"),
                crossterm::event::MediaKeyCode::PlayPause => write!(f, "<MediaPlayPause>"),
                crossterm::event::MediaKeyCode::Reverse => write!(f, "<MediaReverse>"),
                crossterm::event::MediaKeyCode::Stop => write!(f, "<MediaStop>"),
                crossterm::event::MediaKeyCode::FastForward => write!(f, "<MediaFastForward>"),
                crossterm::event::MediaKeyCode::Rewind => write!(f, "<MediaRewind>"),
                crossterm::event::MediaKeyCode::TrackNext => write!(f, "<MediaTrackNext>"),
                crossterm::event::MediaKeyCode::TrackPrevious => write!(f, "<MediaTrackPrevious>"),
                crossterm::event::MediaKeyCode::Record => write!(f, "<MediaRecord>"),
                crossterm::event::MediaKeyCode::LowerVolume => write!(f, "<VolumeDown>"),
                crossterm::event::MediaKeyCode::RaiseVolume => write!(f, "<VolumeUp>"),
                crossterm::event::MediaKeyCode::MuteVolume => write!(f, "<VolumeMute>"),
            },
            crossterm::event::KeyCode::Modifier(code) => match code {
                crossterm::event::ModifierKeyCode::LeftShift => write!(f, "<S-Left>"),
                crossterm::event::ModifierKeyCode::LeftControl => write!(f, "<C-Left>"),
                crossterm::event::ModifierKeyCode::LeftAlt => write!(f, "<M-Left>"),
                crossterm::event::ModifierKeyCode::LeftSuper => write!(f, "<D-Left>"),
                crossterm::event::ModifierKeyCode::LeftHyper => write!(f, "<H-Left>"),
                crossterm::event::ModifierKeyCode::LeftMeta => write!(f, "<Meta-Left>"),
                crossterm::event::ModifierKeyCode::RightShift => write!(f, "<S-Right>"),
                crossterm::event::ModifierKeyCode::RightControl => write!(f, "<C-Right>"),
                crossterm::event::ModifierKeyCode::RightAlt => write!(f, "<M-Right>"),
                crossterm::event::ModifierKeyCode::RightSuper => write!(f, "<D-Right>"),
                crossterm::event::ModifierKeyCode::RightHyper => write!(f, "<H-Right>"),
                crossterm::event::ModifierKeyCode::RightMeta => write!(f, "<Meta-Right>"),
                crossterm::event::ModifierKeyCode::IsoLevel3Shift => {
                    write!(f, "<ISO_Level3_Shift>")
                }
                crossterm::event::ModifierKeyCode::IsoLevel5Shift => {
                    write!(f, "<ISO_Level5_Shift>")
                }
            },
        }
    }
}
