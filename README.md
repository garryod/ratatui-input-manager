# Ratatui Input Manager

A small utility crate providing a declarative approach to handling inputs in [ratatui](https://crates.io/crates/ratatui).

## Usage

```toml
[dependencies]
ratatui-input-manager = { version = "0.1.0", features = ["crossterm", "widgets"] }
```

### Keymap Macro

```rust
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, KeyCode},
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    ExecutableCommand,
};
use ratatui_core::{
    backend::TestBackend,
    layout::{Constraint, Layout},
    terminal::{Frame, Terminal},
    text::Line,
    widgets::Widget,
};
use ratatui_input_manager::{keymap, KeyMap};
use ratatui_input_manager::widgets::HelpBar;

struct App {
    counter: i32,
    quit: bool
}

#[keymap(backend = "crossterm")]
impl App {
    /// Increment the counter
    #[keybind(pressed = KeyCode::Char('+'))]
    fn increment(&mut self) {
        self.counter += 1;
    }

    /// Decrement the counter
    #[keybind(pressed = KeyCode::Char('-'))]
    fn decrement(&mut self) {
        self.counter -= 1;
    }

    /// Exit the application
    #[keybind(pressed = KeyCode::Esc)]
    #[keybind(pressed = KeyCode::Char('q'))]
    fn quit(&mut self) {
        self.quit = true
    }
}

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(TestBackend::new(80, 32)).unwrap();

    // `quit` set to `true` to ensure test exits
    let mut app = App { counter: 0, quit: true };

    loop {
        terminal.draw(|frame: &mut Frame| {
            let [counter_area, help_bar_area] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(1),
            ]).areas(frame.area());
            frame.render_widget(
                Line::from(format!("Counter: {}", app.counter)).centered(),
                counter_area
            );

            frame.render_widget(
                HelpBar::new(App::KEYBINDS),
                help_bar_area,
            );
        }).unwrap();

        if event::poll(Duration::from_millis(10))? {
            let pressed = event::read()?;
            app.handle(&pressed);
        }

        if app.quit {
            break Ok(());
        }
    }
}
```

## License

This project is licensed under the MIT license and the Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for details.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
