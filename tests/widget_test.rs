use crossterm::event::KeyCode;
use ratatui_core::{buffer::Buffer, layout::Rect, widgets::Widget};
use ratatui_input_manager::widgets::{Help, HelpBar};
use ratatui_input_manager::{keymap, KeyMap};

#[derive(Default)]
struct App;

#[keymap(backend = "crossterm")]
impl App {
    #[keybind(pressed(key=KeyCode::Char('q')))]
    fn quit(&mut self) {}
}

#[test]
fn help_widget_renders() {
    let area = Rect::new(0, 0, 20, 5);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS);
    help.render(area, &mut buffer);

    assert!(!buffer.area().is_empty());
}

#[test]
fn help_widget_with_block_renders() {
    use ratatui_widgets::block::Block;

    let area = Rect::new(0, 0, 30, 10);
    let mut buffer = Buffer::empty(area);

    let help = Help::new(App::KEYBINDS).block(Block::default().title("Controls"));
    help.render(area, &mut buffer);

    assert!(!buffer.area().is_empty());
}

#[test]
fn help_bar_widget_renders() {
    let area = Rect::new(0, 0, 20, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar = HelpBar::new(App::KEYBINDS);
    help_bar.render(area, &mut buffer);

    assert!(!buffer.area().is_empty());
}

#[test]
fn help_bar_with_styles_renders() {
    use ratatui_core::style::Style;

    let area = Rect::new(0, 0, 30, 1);
    let mut buffer = Buffer::empty(area);

    let help_bar = HelpBar::new(App::KEYBINDS)
        .key_style(Style::default())
        .description_style(Style::default())
        .separator_style(Style::default());
    help_bar.render(area, &mut buffer);

    assert!(!buffer.area().is_empty());
}
