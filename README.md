# Ratatui Input Manager

<div align="center">

[![Crate Badge]][Crate] [![Repo Badge]][Repo] [![Docs Badge]][Docs]   \
![License Badge] [![CI Badge]][CI] [![Deps Badge]][Deps]

</div>

A small utility crate providing a declarative approach to handling inputs in [ratatui].

## Usage

```toml
[dependencies]
ratatui-input-manager = { version = "0.1.0", features = ["crossterm", "widgets"] }
```

### Keymap Macro

```rust
use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui_core::{
    backend::TestBackend,
    layout::{Constraint, Layout},
    terminal::{Frame, Terminal},
    text::Line,
    widgets::Widget,
};
use ratatui_input_manager::{keymap, KeyMap};
use ratatui_input_manager::widgets::HelpBar;

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

const TEST_EVENTS: &[Event] = &[
    Event::Key(KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE)),
    Event::Key(KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE)),
    Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
];

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(TestBackend::new(80, 32)).unwrap();

    let mut app = App::default();
    let mut events = TEST_EVENTS.iter();

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
            // Renders as: "Increment the counter: <+> | Decrement the counter: <-> | Exit: <Esc>, <q>, <Ctrl+c>"
        }).unwrap();

        /// use [`crossterm::event::poll`] in place of iterating through test
        /// events
        if let Some(event) = events.next() {
            app.handle(event);
        }

        if app.quit {
            break Ok(());
        }
    }
}
```

### Widgets

This crate provides two widgets for displaying keybindings:

```rust
use crossterm::event::KeyCode;
use ratatui_core::terminal::Frame;
use ratatui_input_manager::{keymap, KeyMap};
use ratatui_input_manager::widgets::{Help, HelpBar};

struct App;

#[keymap(backend = "crossterm")]
impl App {
    /// Increment the counter
    #[keybind(pressed(key=KeyCode::Char('+')))]
    fn increment(&mut self) {}

    /// Decrement the counter
    #[keybind(pressed(key=KeyCode::Char('-')))]
    fn decrement(&mut self) {}
}

fn render_help(frame: &mut Frame) {
    let help = Help::new(App::KEYBINDS);
    frame.render_widget(help, frame.area());
}

fn render_help_bar(frame: &mut Frame) {
    let help_bar = HelpBar::new(App::KEYBINDS);
    frame.render_widget(help_bar, frame.area());
}
```

The `Help` widget displays keybindings in a table format:

```text
┌────────────────────────────────────────────┐
│  +  Increment the counter                  │
│  -  Decrement the counter                  │
└────────────────────────────────────────────┘
```

The `HelpBar` widget displays keybindings in a single line:

```text
Increment the counter: <+> | Decrement the counter: <->
```

## License

This project is licensed under the MIT license and the Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for details.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[Crate]: https://crates.io/crates/ratatui-input-manager
[Repo]: https://github.com/garryod/ratatui-input-manager
[Docs]: https://docs.rs/ratatui-input-manager
[CI]: https://github.com/garryod/ratatui-input-manager/actions/workflows/ci.yaml
[Deps]: https://deps.rs/repo/github/garryod/ratatui-input-manager

[Crate Badge]: https://img.shields.io/crates/v/ratatui-input-manager?logo=rust&style=flat-square&color=E05D44
[Repo Badge]: https://img.shields.io/badge/repo-garryod/ratatui--input--manaer-1370D3?style=flat-square&logo=github
[Docs Badge]: https://img.shields.io/badge/docs-ratatui--input--manager-1370D3?style=flat-square&logo=rust
[License Badge]: https://img.shields.io/crates/l/ratatui-input-manager?style=flat-square&color=1370D3
[CI Badge]: https://img.shields.io/github/actions/workflow/status/garryod/ratatui-input-manager/ci.yaml?style=flat-square&logo=github
[Deps Badge]: https://deps.rs/repo/github/garryod/ratatui-input-manager/status.svg?style=flat-square

[Ratatui]: https://crates.io/crates/ratatui
