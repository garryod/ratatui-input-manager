use crossterm::event::KeyCode;
use ratatui_core::{buffer::Buffer, layout::Rect, style::Color, style::Style, widgets::Widget};
use ratatui_input_manager::widgets::{Help, HelpBar};
use ratatui_input_manager::{KeyMap, keymap};

#[derive(Default)]
struct App;

#[keymap(backend = "crossterm")]
impl App {
    /// Quit the application
    #[keybind(pressed(key = KeyCode::Char('q')))]
    fn quit(&mut self) {}
}

#[test]
fn help_widget_default_renders() {
    let area = Rect::new(0, 0, 24, 1);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS);
    help.render(area, &mut buffer);

    let expected = Buffer::with_lines(vec!["<q> Quit the application"]);

    assert_eq!(buffer, expected);
}

#[test]
fn help_widget_with_style() {
    let area = Rect::new(0, 0, 24, 1);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS).style(Style::default().fg(Color::Red));
    help.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["<q> Quit the application"]);
    expected.set_style(Rect::new(0, 0, 24, 1), Style::default().fg(Color::Red));

    assert_eq!(buffer, expected);
}

#[test]
fn help_widget_with_key_style() {
    let area = Rect::new(0, 0, 24, 1);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS).key_style(Style::default().fg(Color::Cyan));
    help.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["<q> Quit the application"]);
    expected.set_style(Rect::new(0, 0, 3, 1), Style::default().fg(Color::Cyan));

    assert_eq!(buffer, expected);
}

#[test]
fn help_widget_with_description_style() {
    let area = Rect::new(0, 0, 24, 1);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS).description_style(Style::default().fg(Color::Green));
    help.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["<q> Quit the application"]);
    expected.set_style(Rect::new(4, 0, 24, 1), Style::default().fg(Color::Green));

    assert_eq!(buffer, expected);
}

#[test]
fn help_widget_with_block() {
    use ratatui_widgets::block::Block;

    let area = Rect::new(0, 0, 26, 3);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS).block(
        Block::default()
            .title("Controls")
            .borders(ratatui_widgets::borders::Borders::ALL),
    );
    help.render(area, &mut buffer);

    let expected = Buffer::with_lines(vec![
        "┌Controls────────────────┐",
        "│<q> Quit the application│",
        "└────────────────────────┘",
    ]);

    assert_eq!(buffer, expected);
}

#[test]
fn help_bar_single_binding() {
    let area = Rect::new(0, 0, 25, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar = HelpBar::new(App::KEYBINDS);
    help_bar.render(area, &mut buffer);

    let expected = Buffer::with_lines(vec!["Quit the application: <q>"]);

    assert_eq!(buffer, expected);
}

#[test]
fn help_bar_with_global_style() {
    let area = Rect::new(0, 0, 25, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar = HelpBar::new(App::KEYBINDS).style(Style::default().fg(Color::Red));
    help_bar.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["Quit the application: <q>"]);
    expected.set_style(Rect::new(0, 0, 25, 1), Style::default().fg(Color::Red));

    assert_eq!(buffer, expected);
}

#[test]
fn help_bar_with_key_style() {
    let area = Rect::new(0, 0, 25, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar = HelpBar::new(App::KEYBINDS).key_style(Style::default().fg(Color::Cyan));
    help_bar.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["Quit the application: <q>"]);
    expected.set_style(Rect::new(22, 0, 3, 1), Style::default().fg(Color::Cyan));

    assert_eq!(buffer, expected);
}

#[test]
fn help_bar_with_description_style() {
    let area = Rect::new(0, 0, 25, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar = HelpBar::new(App::KEYBINDS).description_style(Style::default().fg(Color::Green));
    help_bar.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["Quit the application: <q>"]);
    expected.set_style(Rect::new(0, 0, 22, 1), Style::default().fg(Color::Green));

    assert_eq!(buffer, expected);
}

#[test]
fn help_bar_with_separator_style() {
    #[derive(Default)]
    struct App2;

    #[keymap(backend = "crossterm")]
    impl App2 {
        /// Add an item
        #[keybind(pressed(key = KeyCode::Char('a')))]
        fn add(&mut self) {}

        /// Delete an item
        #[keybind(pressed(key = KeyCode::Char('d')))]
        fn delete(&mut self) {}
    }

    let area = Rect::new(0, 0, 38, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar =
        HelpBar::new(App2::KEYBINDS).separator_style(Style::default().fg(Color::DarkGray));
    help_bar.render(area, &mut buffer);

    let mut expected = Buffer::with_lines(vec!["Add an item: <a> | Delete an item: <d>"]);
    expected.set_style(Rect::new(16, 0, 3, 1), Style::default().fg(Color::DarkGray));

    assert_eq!(buffer, expected);
}
