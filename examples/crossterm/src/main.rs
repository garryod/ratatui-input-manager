use std::{io, time::Duration};

use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
};
use ratatui_input_manager::widgets::HelpBar;
use ratatui_input_manager::{KeyMap, keymap};

#[derive(Default)]
struct App {
    counter: i32,
    quit: bool,
}

#[keymap(backend = "crossterm")]
impl App {
    /// Increment the counter
    #[keybind(pressed(key=KeyCode::Char('+')))]
    fn increment(&mut self) {
        self.counter += 1;
    }

    /// Decrement the counter
    #[keybind(pressed(key=KeyCode::Char('-')))]
    fn decrement(&mut self) {
        self.counter -= 1;
    }

    /// Exit the application
    #[keybind(pressed(key=KeyCode::Esc))]
    #[keybind(pressed(key=KeyCode::Char('q')))]
    #[keybind(pressed(key=KeyCode::Char('c'), modifiers=KeyModifiers::CONTROL))]
    fn quit(&mut self) {
        self.quit = true
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::default();

    loop {
        terminal
            .draw(|frame: &mut Frame| {
                let [counter_area, help_bar_area] =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
                        .areas(frame.area());
                frame.render_widget(
                    Line::from(format!("Counter: {}", app.counter)).centered(),
                    counter_area,
                );

                frame.render_widget(HelpBar::new(App::KEYBINDS), help_bar_area);
            })
            .unwrap();

        if event::poll(Duration::from_millis(10)).unwrap() {
            app.handle(&event::read().unwrap());
        }

        if app.quit {
            break Ok(());
        }
    }
}
