use std::sync::Arc;

use ratatui::prelude::{Line, Text};
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui::{layout, widgets};

const TABS: [&str; 6] = [
    "Hardware", "Software", "Storage", "Modify", "Review", "Write",
];

/// One row of the board list.
#[derive(Default, Debug, Clone)]
pub struct Board {
    pub id: i64,
    pub name: Box<str>,
    pub description: Arc<str>,
}

impl<'a> From<&'a Board> for widgets::ListItem<'a> {
    fn from(value: &'a Board) -> Self {
        Self::new(Text::from_iter([
            Line::from(value.name.as_ref()),
            Line::from(value.description.as_ref()),
        ]))
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub boards: Box<[Board]>,
}

impl widgets::Widget for &State {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout =
            layout::Layout::vertical([layout::Constraint::Length(2), layout::Constraint::Fill(1)]);

        let [header, body] = layout.areas(area);

        widgets::Tabs::new(TABS)
            .block(widgets::Block::new().borders(widgets::Borders::BOTTOM))
            .render(header, buf);

        widgets::List::new(&self.boards).render(body, buf);
    }
}
