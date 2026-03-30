#![cfg(feature = "termwiz")]
use ratatui_input_manager::{keymap, KeyMap};
use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    ctrl_shift_a: bool,
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
    assert_eq!(TestKeyMap::KEYBINDS.len(), 2);
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
