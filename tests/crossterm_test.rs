#![cfg(feature = "crossterm")]
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui_input_manager::{KeyMap, keymap};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestConfirmationOverlayKeyMap {
    accepted: Option<bool>,
}

#[keymap(backend = "crossterm")]
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
    submitted: bool,
}

impl TestKeyMap {
    fn with_active_overlay() -> TestKeyMap {
        TestKeyMap {
            confirmation_overlay: Some(TestConfirmationOverlayKeyMap { accepted: None }),
            ..Default::default()
        }
    }

    fn handle(&mut self, event: &Event) -> bool {
        if let Some(overlay) = &mut self.confirmation_overlay {
            let consumed = overlay.handle(event);

            if let Some(accepted) = overlay.accepted {
                self.confirmation_overlay = None;
                if accepted {
                    self.submit();
                }
            }

            if consumed {
                return true;
            }
        }

        KeyMap::handle(self, event)
    }

    fn submit(&mut self) {
        self.submitted = true;
    }
}

#[keymap(backend = "crossterm")]
impl TestKeyMap {
    /// Handle escape or q to exit
    #[keybind(pressed(key=KeyCode::Esc))]
    #[keybind(pressed(key=KeyCode::Char('q')))]
    fn handle_esc(&mut self) {
        self.exit = true;
    }

    /// Handle Ctrl+Shift+A
    #[keybind(pressed(key=KeyCode::Char('a'), modifiers=KeyModifiers::CONTROL, modifiers=KeyModifiers::SHIFT))]
    fn handle_ctrl_shift_a(&mut self) {
        self.ctrl_shift_a = true;
    }

    /// Handle Y
    #[keybind(pressed(key=KeyCode::Char('y')))]
    fn handle_y(&mut self) {
        self.y = true;
    }
}

#[test]
fn test_handle_escape() {
    let mut map = TestKeyMap::default();

    let event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(map.handle(&event));

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

    let event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    assert!(map.handle(&event));

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

    let event = Event::Key(KeyEvent::new(
        KeyCode::Char('a'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(map.handle(&event));

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

    let event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    assert!(!map.handle(&event));

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
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[0].key, KeyCode::Esc);
    assert_eq!(
        TestKeyMap::KEYBINDS[0].pressed[0].modifiers,
        KeyModifiers::NONE
    );
    assert_eq!(TestKeyMap::KEYBINDS[0].pressed[1].key, KeyCode::Char('q'));
    assert_eq!(
        TestKeyMap::KEYBINDS[0].pressed[1].modifiers,
        KeyModifiers::NONE
    );
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed.len(), 1);
    assert_eq!(TestKeyMap::KEYBINDS[1].pressed[0].key, KeyCode::Char('a'));
    assert_eq!(
        TestKeyMap::KEYBINDS[1].pressed[0].modifiers,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT
    );
}

#[test]
fn test_overlay_keybinds() {
    let mut map = TestKeyMap::with_active_overlay();

    // Root handles non-conflicting keys with overlay active
    let event = Event::Key(KeyEvent::new(
        KeyCode::Char('a'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(map.handle(&event));
    assert!(map.ctrl_shift_a);

    // Overlay consumes conflicting key, submits, and is dismissed
    let event = Event::Key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    assert!(map.handle(&event));
    assert!(!map.y);
    assert!(map.submitted);
    assert!(map.confirmation_overlay.is_none());

    // With overlay dismissed, conflicting keys pass through to root
    let event = Event::Key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    assert!(map.handle(&event));
    assert!(map.y);
}
