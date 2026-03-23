#![cfg(feature = "termion")]
use ratatui_input_manager::{KeyMap, keymap};
use termion::event::{Event, Key};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    a: bool,
}

#[keymap(backend = "termion")]
impl TestKeyMap {
    #[keybind(pressed = Key::Esc)]
    #[keybind(pressed = Key::Char('q'))]
    fn handle_esc(&mut self) {
        self.exit = true;
    }

    #[keybind(pressed = Key::Char('a'))]
    fn handle_a(&mut self) {
        self.a = true
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
    assert_eq!(TestKeyMap::KEYBINDS.len(), 2);
    assert_eq!(TestKeyMap::KEYBINDS[0].keys, &[Key::Esc, Key::Char('q')]);
    assert_eq!(TestKeyMap::KEYBINDS[1].keys, &[Key::Char('a')]);
}
