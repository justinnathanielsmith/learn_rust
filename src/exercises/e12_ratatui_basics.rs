use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::rc::Rc;

/// Create a Paragraph widget that displays the text "Welcome to Ratatui!".
/// It should not have any explicit styling or blocks for this exercise.
pub fn welcome_paragraph() -> Paragraph<'static> {
    todo!("Implement welcome_paragraph")
}

/// Create a layout that splits the given `area` into three vertical chunks:
/// 1. Header: 3 lines high
/// 2. Main: Remaining space
/// 3. Footer: 3 lines high
pub fn create_app_layout(area: Rect) -> Rc<[Rect]> {
    todo!("Implement create_app_layout")
}

/// Create a Block widget with the title "Ratatui Exercise" and all borders (All).
pub fn styled_block() -> Block<'static> {
    todo!("Implement styled_block")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_welcome_paragraph() {
        let backend = TestBackend::new(30, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        let widget = welcome_paragraph();
        
        terminal.draw(|f| {
            f.render_widget(widget, f.size());
        }).unwrap();

        let expected_content = "Welcome to Ratatui!           ";
        let buffer = terminal.backend().buffer();
        let mut actual_content = String::new();
        for x in 0..30 {
            actual_content.push(buffer.get(x, 0).symbol().chars().next().unwrap_or(' '));
        }
        
        assert_eq!(actual_content.trim(), "Welcome to Ratatui!");
    }

    #[test]
    fn test_create_app_layout() {
        let area = Rect::new(0, 0, 10, 10);
        let chunks = create_app_layout(area);
        
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].height, 3, "Header should be 3 lines high");
        assert_eq!(chunks[2].height, 3, "Footer should be 3 lines high");
        assert_eq!(chunks[1].height, 4, "Main should be the remaining 4 lines");
    }

    #[test]
    fn test_styled_block() {
        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let block = styled_block();
        
        terminal.draw(|f| {
            let area = f.size();
            f.render_widget(block, area);
        }).unwrap();

        let buffer = terminal.backend().buffer();
        
        // Check for title "Ratatui Exercise"
        let mut title = String::new();
        for x in 2..19 { // Title usually starts after some padding/border
            let ch = buffer.get(x, 0).symbol();
            if ch != "─" && ch != "┌" && ch != "┐" {
                title.push_str(ch);
            }
        }
        assert!(title.contains("Ratatui Exercise"), "Block should contain the correct title. Found: {}", title);
        
        // Check for borders (top left corner)
        assert_eq!(buffer.get(0, 0).symbol(), "┌");
        assert_eq!(buffer.get(19, 0).symbol(), "┐");
        assert_eq!(buffer.get(0, 2).symbol(), "└");
        assert_eq!(buffer.get(19, 2).symbol(), "┘");
    }
}
