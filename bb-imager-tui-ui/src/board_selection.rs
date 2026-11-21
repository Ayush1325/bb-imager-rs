use ratatui::widgets;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct State {}

impl widgets::Widget for &State {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        "Hello World".render(area, buf);
    }
}
