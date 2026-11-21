use bb_imager_tui_ui::board_selection;
use ratatui::crossterm::event;
use ratatui::widgets;

fn main() -> std::io::Result<()> {
    ratatui::run(|t| {
        loop {
            let state = board_selection::State {};
            t.draw(|f| {
                let outer_block = widgets::Block::bordered().title("BeagleBoard Imager");

                let outer_area = f.area();
                let inner_area = outer_block.inner(outer_area);

                f.render_widget(outer_block, outer_area);
                f.render_widget(&state, inner_area)
            })?;

            match event::read()? {
                _ => return Ok(()),
            }
        }
    })
}
