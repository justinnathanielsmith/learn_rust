# Lesson 12: Terminal UIs with Ratatui

Ratatui is a library for building rich terminal user interfaces (TUIs) in Rust.

## Basic Widgets: `Paragraph`

A `Paragraph` is a simple widget that displays text.

```rust
use ratatui::widgets::{Block, Borders, Paragraph};

let p = Paragraph::new("Hello, Ratatui!")
    .block(Block::default().title("Greeting").borders(Borders::ALL));
```

## Styling: `Block` and `Style`

A `Block` defines the area where a widget is drawn, typically with borders and a title.

```rust
let block = Block::default()
    .title("My Block")
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::Cyan).bg(Color::Black));
```

## Layout: `Layout` and `Rect`

Ratatui uses a powerful layout engine to divide the terminal screen into rectangular areas (`Rect`).

```rust
use ratatui::layout::{Constraint, Direction, Layout, Rect};

fn create_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Bottom bar
        ])
        .split(area)
}
```

- `Constraint::Length(n)`: Fixed size.
- `Constraint::Percentage(n)`: Percentage of available space.
- `Constraint::Min(n)`: Grow to take up remaining space.

## Further Reading

- [Ratatui Website](https://ratatui.rs)
- [Ratatui Documentation](https://docs.rs/ratatui/latest/ratatui/)
