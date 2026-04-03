#![cfg(feature = "crossterm")]
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui_input_manager::{KeyMap, keymap};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    ctrl_shift_a: bool,
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
    assert_eq!(TestKeyMap::KEYBINDS.len(), 2);
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
