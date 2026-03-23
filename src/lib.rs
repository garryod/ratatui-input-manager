#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Ratatui widgets displaying keybinds
#[cfg(feature = "widgets")]
pub mod widgets;

pub use ratatui_input_manager_derive::keymap;
use std::fmt::Display;

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

#[cfg(feature = "termion")]
impl<'k> Display for Vimlike<'k, termion::event::Key> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            termion::event::Key::Backspace => write!(f, "<BS>"),
            termion::event::Key::Left => write!(f, "<Left>"),
            termion::event::Key::ShiftLeft => write!(f, "<S-Left>"),
            termion::event::Key::AltLeft => write!(f, "<M-Left>"),
            termion::event::Key::CtrlLeft => write!(f, "<C-Left>"),
            termion::event::Key::Right => write!(f, "<Right>"),
            termion::event::Key::ShiftRight => write!(f, "<S-Right>"),
            termion::event::Key::AltRight => write!(f, "<M-Right>"),
            termion::event::Key::CtrlRight => write!(f, "<C-Right>"),
            termion::event::Key::Up => write!(f, "<Up>"),
            termion::event::Key::ShiftUp => write!(f, "<S-Up>"),
            termion::event::Key::AltUp => write!(f, "<M-Up>"),
            termion::event::Key::CtrlUp => write!(f, "<C-Up>"),
            termion::event::Key::Down => write!(f, "<Down>"),
            termion::event::Key::ShiftDown => write!(f, "<S-Down>"),
            termion::event::Key::AltDown => write!(f, "<M-Down>"),
            termion::event::Key::CtrlDown => write!(f, "<C-Down>"),
            termion::event::Key::Home => write!(f, "<Home>"),
            termion::event::Key::CtrlHome => write!(f, "<C-Home>"),
            termion::event::Key::End => write!(f, "<End>"),
            termion::event::Key::CtrlEnd => write!(f, "<C-End>"),
            termion::event::Key::PageUp => write!(f, "<PageUp>"),
            termion::event::Key::PageDown => write!(f, "<PageDown>"),
            termion::event::Key::BackTab => write!(f, "<S-Tab>"),
            termion::event::Key::Delete => write!(f, "<Del>"),
            termion::event::Key::Insert => write!(f, "<Insert>"),
            termion::event::Key::F(num) => write!(f, "<F{num}>"),
            termion::event::Key::Char(c) => write!(f, "{c}"),
            termion::event::Key::Alt(c) => write!(f, "<M-{c}>"),
            termion::event::Key::Ctrl(c) => write!(f, "<C-{c}>"),
            termion::event::Key::Null => write!(f, "<Nul>"),
            termion::event::Key::Esc => write!(f, "<Esc>"),
            _ => write!(f, "<Unknown>"),
        }
    }
}

#[cfg(feature = "termwiz")]
impl<'k> Display for Vimlike<'k, termwiz::input::KeyCode> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            termwiz::input::KeyCode::Char(c) => write!(f, "{c}"),
            termwiz::input::KeyCode::Hyper => write!(f, "<Hyper>"),
            termwiz::input::KeyCode::Super => write!(f, "<Super>"),
            termwiz::input::KeyCode::Meta => write!(f, "<M>"),
            termwiz::input::KeyCode::Cancel => write!(f, "<Cancel>"),
            termwiz::input::KeyCode::Backspace => write!(f, "<BS>"),
            termwiz::input::KeyCode::Tab => write!(f, "<Tab>"),
            termwiz::input::KeyCode::Clear => write!(f, "<Clear>"),
            termwiz::input::KeyCode::Enter => write!(f, "<CR>"),
            termwiz::input::KeyCode::Shift => write!(f, "<S>"),
            termwiz::input::KeyCode::Escape => write!(f, "<Esc>"),
            termwiz::input::KeyCode::LeftShift => write!(f, "<S-Left>"),
            termwiz::input::KeyCode::RightShift => write!(f, "<S-Right>"),
            termwiz::input::KeyCode::Control => write!(f, "<C>"),
            termwiz::input::KeyCode::LeftControl => write!(f, "<C-Left>"),
            termwiz::input::KeyCode::RightControl => write!(f, "<C-Right>"),
            termwiz::input::KeyCode::Alt => write!(f, "<M>"),
            termwiz::input::KeyCode::LeftAlt => write!(f, "<M-Left>"),
            termwiz::input::KeyCode::RightAlt => write!(f, "<M-Right>"),
            termwiz::input::KeyCode::Menu => write!(f, "<Menu>"),
            termwiz::input::KeyCode::LeftMenu => write!(f, "<Menu-Left>"),
            termwiz::input::KeyCode::RightMenu => write!(f, "<Menu-Right>"),
            termwiz::input::KeyCode::Pause => write!(f, "<Pause>"),
            termwiz::input::KeyCode::CapsLock => write!(f, "<CapsLock>"),
            termwiz::input::KeyCode::PageUp => write!(f, "<PageUp>"),
            termwiz::input::KeyCode::PageDown => write!(f, "<PageDown>"),
            termwiz::input::KeyCode::End => write!(f, "<End>"),
            termwiz::input::KeyCode::Home => write!(f, "<Home>"),
            termwiz::input::KeyCode::LeftArrow => write!(f, "<Left>"),
            termwiz::input::KeyCode::RightArrow => write!(f, "<Right>"),
            termwiz::input::KeyCode::UpArrow => write!(f, "<Up>"),
            termwiz::input::KeyCode::DownArrow => write!(f, "<Down>"),
            termwiz::input::KeyCode::Select => write!(f, "<Select>"),
            termwiz::input::KeyCode::Print => write!(f, "<Print>"),
            termwiz::input::KeyCode::Execute => write!(f, "<Execute>"),
            termwiz::input::KeyCode::PrintScreen => write!(f, "<PrintScreen>"),
            termwiz::input::KeyCode::Insert => write!(f, "<Insert>"),
            termwiz::input::KeyCode::Delete => write!(f, "<Del>"),
            termwiz::input::KeyCode::Help => write!(f, "<Help>"),
            termwiz::input::KeyCode::LeftWindows => write!(f, "<Windows-Left>"),
            termwiz::input::KeyCode::RightWindows => write!(f, "<Windows-Right>"),
            termwiz::input::KeyCode::Applications => write!(f, "<Apps>"),
            termwiz::input::KeyCode::Sleep => write!(f, "<Sleep>"),
            termwiz::input::KeyCode::Numpad0 => write!(f, "<k0>"),
            termwiz::input::KeyCode::Numpad1 => write!(f, "<k1>"),
            termwiz::input::KeyCode::Numpad2 => write!(f, "<k2>"),
            termwiz::input::KeyCode::Numpad3 => write!(f, "<k3>"),
            termwiz::input::KeyCode::Numpad4 => write!(f, "<k4>"),
            termwiz::input::KeyCode::Numpad5 => write!(f, "<k5>"),
            termwiz::input::KeyCode::Numpad6 => write!(f, "<k6>"),
            termwiz::input::KeyCode::Numpad7 => write!(f, "<k7>"),
            termwiz::input::KeyCode::Numpad8 => write!(f, "<k8>"),
            termwiz::input::KeyCode::Numpad9 => write!(f, "<k9>"),
            termwiz::input::KeyCode::Multiply => write!(f, "<kMultiply>"),
            termwiz::input::KeyCode::Add => write!(f, "<kAdd>"),
            termwiz::input::KeyCode::Separator => write!(f, "<kSeparator>"),
            termwiz::input::KeyCode::Subtract => write!(f, "<kSubtract>"),
            termwiz::input::KeyCode::Decimal => write!(f, "<kDecimal>"),
            termwiz::input::KeyCode::Divide => write!(f, "<kDivide>"),
            termwiz::input::KeyCode::Function(n) => write!(f, "<F{n}>"),
            termwiz::input::KeyCode::NumLock => write!(f, "<NumLock>"),
            termwiz::input::KeyCode::ScrollLock => write!(f, "<ScrollLock>"),
            termwiz::input::KeyCode::Copy => write!(f, "<Copy>"),
            termwiz::input::KeyCode::Cut => write!(f, "<Cut>"),
            termwiz::input::KeyCode::Paste => write!(f, "<Paste>"),
            termwiz::input::KeyCode::BrowserBack => write!(f, "<BrowserBack>"),
            termwiz::input::KeyCode::BrowserForward => write!(f, "<BrowserForward>"),
            termwiz::input::KeyCode::BrowserRefresh => write!(f, "<BrowserRefresh>"),
            termwiz::input::KeyCode::BrowserStop => write!(f, "<BrowserStop>"),
            termwiz::input::KeyCode::BrowserSearch => write!(f, "<BrowserSearch>"),
            termwiz::input::KeyCode::BrowserFavorites => write!(f, "<BrowserFavorites>"),
            termwiz::input::KeyCode::BrowserHome => write!(f, "<BrowserHome>"),
            termwiz::input::KeyCode::VolumeMute => write!(f, "<VolumeMute>"),
            termwiz::input::KeyCode::VolumeDown => write!(f, "<VolumeDown>"),
            termwiz::input::KeyCode::VolumeUp => write!(f, "<VolumeUp>"),
            termwiz::input::KeyCode::MediaNextTrack => write!(f, "<MediaNextTrack>"),
            termwiz::input::KeyCode::MediaPrevTrack => write!(f, "<MediaPrevTrack>"),
            termwiz::input::KeyCode::MediaStop => write!(f, "<MediaStop>"),
            termwiz::input::KeyCode::MediaPlayPause => write!(f, "<MediaPlayPause>"),
            termwiz::input::KeyCode::ApplicationLeftArrow => write!(f, "<App-Left>"),
            termwiz::input::KeyCode::ApplicationRightArrow => write!(f, "<App-Right>"),
            termwiz::input::KeyCode::ApplicationUpArrow => write!(f, "<App-Up>"),
            termwiz::input::KeyCode::ApplicationDownArrow => write!(f, "<App-Down>"),
            termwiz::input::KeyCode::KeyPadHome => write!(f, "<kHome>"),
            termwiz::input::KeyCode::KeyPadEnd => write!(f, "<kEnd>"),
            termwiz::input::KeyCode::KeyPadPageUp => write!(f, "<kPageUp>"),
            termwiz::input::KeyCode::KeyPadPageDown => write!(f, "<kPageDown>"),
            termwiz::input::KeyCode::KeyPadBegin => write!(f, "<kBegin>"),
            _ => write!(f, "<Unknown>"),
        }
    }
}
