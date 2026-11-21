use bb_imager_tui_ui::board_selection;
use ratatui::crossterm::event;
use ratatui::widgets;

fn main() -> std::io::Result<()> {
    let cfg =
        serde_json::from_slice::<bb_config::config::Config>(include_bytes!("../../../config.json"))
            .expect("Failed to parse config");
    let boards = cfg
        .imager
        .devices
        .iter()
        .enumerate()
        .map(|(id, x)| board_selection::Board {
            id: id as i64,
            name: x.name.clone(),
            description: x.description.clone().into(),
        })
        .collect();

    let state = board_selection::State { boards };

    ratatui::run(|t| {
        t.draw(|f| {
            let outer_block = widgets::Block::bordered().title("BeagleBoard Imager");

            let outer_area = f.area();
            let inner_area = outer_block.inner(outer_area);

            f.render_widget(outer_block, outer_area);
            f.render_widget(&state, inner_area)
        })?;

        event::read()?;
        Ok(())
    })
}
