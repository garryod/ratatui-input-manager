#![cfg(feature = "termwiz")]
use ratatui_input_manager::{KeyMap, keymap};
use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    a: bool,
}

#[keymap(backend = "termwiz")]
impl TestKeyMap {
    #[keybind(pressed = KeyCode::Escape)]
    #[keybind(pressed = KeyCode::Char('q'))]
    fn handle_esc(&mut self) {
        self.exit = true;
    }

    #[keybind(pressed = KeyCode::Char('a'))]
    fn handle_a(&mut self) {
        self.a = true
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
fn test_handle_a() {
    let mut map = TestKeyMap::default();

    let event = InputEvent::Key(KeyEvent {
        key: KeyCode::Char('a'),
        modifiers: Modifiers::NONE,
    });
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
    assert_eq!(TestKeyMap::KEYBINDS.len(), 2);
    assert_eq!(
        TestKeyMap::KEYBINDS[0].keys,
        &[KeyCode::Escape, KeyCode::Char('q')]
    );
    assert_eq!(TestKeyMap::KEYBINDS[1].keys, &[KeyCode::Char('a')]);
}
