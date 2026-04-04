#![cfg(feature = "termwiz")]
use ratatui_input_manager::{KeyMap, keymap};
use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestConfirmationOverlayKeyMap {
    accepted: Option<bool>,
}

#[keymap(backend = "termwiz")]
impl TestConfirmationOverlayKeyMap {
    /// Handle y to accept
    #[keybind(pressed(key=KeyCode::Char('y')))]
    fn handle_accept(&mut self) {
        self.accepted = Some(true);
    }

    /// Handle n to reject
    #[keybind(pressed(key=KeyCode::Char('n')))]
    fn handle_reject(&mut self) {
        self.accepted = Some(false);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    ctrl_shift_a: bool,
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

    fn handle(&mut self, event: &InputEvent) -> bool {
        if let Some(overlay) = &mut self.confirmation_overlay
            && overlay.handle(event) {
                self.confirmation_overlay = None;
                return true;
            }

        KeyMap::handle(self, event)
    }
}

#[keymap(backend = "termwiz")]
impl TestKeyMap {
    /// Handle escape or q to exit
    #[keybind(pressed(key=KeyCode::Escape))]
    #[keybind(pressed(key=KeyCode::Char('q')))]
    fn handle_esc(&mut self) {
        self.exit = true;
    }

    /// Handle Ctrl+Shift+A
    #[keybind(pressed(key=KeyCode::Char('a'), modifiers=Modifiers::CTRL, modifiers=Modifiers::SHIFT))]
    fn handle_ctrl_shift_a(&mut self) {
        self.ctrl_shift_a = true;
    }

    /// Handle y key
    #[keybind(pressed(key=KeyCode::Char('y')))]
    fn handle_y(&mut self) {
        self.y = true;
    }
}

#[test]
fn test_handle_escape() {
    let mut map = TestKeyMap::default();

    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Escape,
        modifiers: Modifiers::NONE,
    });
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

    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Char('q'),
        modifiers: Modifiers::NONE,
    });
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
fn test_handle_ctrl_shift_a() {
    let mut map = TestKeyMap::default();

    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Char('a'),
        modifiers: Modifiers::CTRL | Modifiers::SHIFT,
    });
    map.handle(&event);

    assert_eq!(
        TestKeyMap {
            ctrl_shift_a: true,
            ..Default::default()
        },
        map
    );
}

#[test]
fn test_ignores_a() {
    let mut map = TestKeyMap::default();

    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Char('a'),
        modifiers: Modifiers::NONE,
    });
    map.handle(&event);

    assert_eq!(
        TestKeyMap {
            ctrl_shift_a: false,
            ..Default::default()
        },
        map
    );
}

#[test]
fn test_keybinds() {
    assert_eq!(TestKeyMap::KEYBINDS.len(), 3);
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed.len(), 2);
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[0].key, KeyCode::Escape,);
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[1].key, KeyCode::Char('q'),);
    assert_eq!(
        TestKeyMap::KEYBINDS[0].pressed[0].modifiers,
        Modifiers::NONE
    );
    assert_eq!(
        TestKeyMap::KEYBINDS[0].pressed[1].modifiers,
        Modifiers::NONE
    );
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed.len(), 1);
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed[0].key, KeyCode::Char('a'));
    assert_eq!(
        TestKeyMap::KEYBINDS[1].pressed[0].modifiers,
        Modifiers::CTRL | Modifiers::SHIFT
    );
}

#[test]
fn test_overlay_keybinds() {
    let mut map = TestKeyMap::with_active_overlay();

    // Root handles non-conflicting keys with overlay active
    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Char('a'),
        modifiers: Modifiers::CTRL | Modifiers::SHIFT,
    });
    assert!(map.handle(&event));
    assert!(map.ctrl_shift_a);

    // Overlay consumes conflicting key and is dismissed
    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Char('y'),
        modifiers: Modifiers::NONE,
    });
    assert!(map.handle(&event));
    assert!(!map.y);

    // With overlay dismissed, conflicting key passes through to root
    assert!(map.handle(&event));
    assert!(map.y);
}
