#![cfg(feature = "termion")]
use ratatui_input_manager::{KeyMap, keymap};
use termion::event::{Event, Key};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestConfirmationOverlayKeyMap {
    accepted: Option<bool>,
}

#[keymap(backend = "termion")]
impl TestConfirmationOverlayKeyMap {
    /// Handle y to accept
    #[keybind(pressed(key = Key::Char('y')))]
    fn handle_accept(&mut self) {
        self.accepted = Some(true);
    }

    /// Handle n to reject
    #[keybind(pressed(key = Key::Char('n')))]
    fn handle_reject(&mut self) {
        self.accepted = Some(false);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    a: bool,
    y: bool,

    confirmation_overlay: Option<TestConfirmationOverlayKeyMap>,
}

impl TestKeyMap {
    fn with_active_overlay() -> TestKeyMap {
        TestKeyMap {
            confirmation_overlay: Some(TestConfirmationOverlayKeyMap { accepted: None }),
            ..Default::default()
        }
    }

    fn handle(&mut self, event: &Event) -> bool {
        if let Some(overlay) = &mut self.confirmation_overlay
            && overlay.handle(event) {
                self.confirmation_overlay = None;
                return true;
            }

        KeyMap::handle(self, event)
    }
}

#[keymap(backend = "termion")]
impl TestKeyMap {
    /// Handle escape or q to exit
    #[keybind(pressed(key = Key::Esc))]
    #[keybind(pressed(key = Key::Char('q')))]
    fn handle_esc(&mut self) {
        self.exit = true;
    }

    /// Handle a key
    #[keybind(pressed(key = Key::Char('a')))]
    fn handle_a(&mut self) {
        self.a = true;
    }

    /// Handle y key
    #[keybind(pressed(key = Key::Char('y')))]
    fn handle_y(&mut self) {
        self.y = true;
    }
}

#[test]
fn test_handle_escape() {
    let mut map = TestKeyMap::default();

    let event = Event::Key(Key::Esc);
    map.handle(&event);

    assert_eq!(
        TestKeyMap {
            exit: true,
            ..Default::default()
        },
        map
    );
}

#[test]
fn test_handle_q() {
    let mut map = TestKeyMap::default();

    let event = Event::Key(Key::Char('q'));
    map.handle(&event);

    assert_eq!(
        TestKeyMap {
            exit: true,
            ..Default::default()
        },
        map
    );
}

#[test]
fn test_handle_a() {
    let mut map = TestKeyMap::default();

    let event = Event::Key(Key::Char('a'));
    map.handle(&event);

    assert_eq!(
        TestKeyMap {
            a: true,
            ..Default::default()
        },
        map
    );
}

#[test]
fn test_keybinds() {
    assert_eq!(TestKeyMap::KEYBINDS.len(), 3);
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed.len(), 2);
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[0].key, Key::Esc);
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[0].modifiers, ());
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[1].key, Key::Char('q'));
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[1].modifiers, ());
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed.len(), 1);
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed[0].key, Key::Char('a'));
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed[0].modifiers, ());
}

#[test]
fn test_overlay_keybinds() {
    let mut map = TestKeyMap::with_active_overlay();

    // Root handles non-conflicting keys with overlay active
    let event = Event::Key(Key::Char('a'));
    assert!(map.handle(&event));
    assert!(map.a);

    // Overlay consumes conflicting key and is dismissed
    let event = Event::Key(Key::Char('y'));
    assert!(map.handle(&event));
    assert!(!map.y);

    // With overlay dismissed, conflicting key passes through to root
    assert!(map.handle(&event));
    assert!(map.y);
}
