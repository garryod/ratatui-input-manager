use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui_input_manager::{KeyMap, keymap};

#[derive(Debug, Default, PartialEq, Eq)]
struct TestKeyMap {
    exit: bool,
    a: bool,
}

#[keymap]
impl TestKeyMap {
    #[keybind(pressed = KeyCode::Esc)]
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

    let event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
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

    let event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
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

    let event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
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
        &[KeyCode::Esc, KeyCode::Char('q')]
    );
    assert_eq!(TestKeyMap::KEYBINDS[1].keys, &[KeyCode::Char('a')]);
}
