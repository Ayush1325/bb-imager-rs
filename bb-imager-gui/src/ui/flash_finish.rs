use iced::{
    Element,
    widget::{self, button},
};

use crate::{
    BBImagerMessage, constants,
    state::FlashingFinishState,
    ui::helpers::{board_view_pane, page_type1, progress_finish_view},
};

/// How a flashing run ended. Both endings show the same page over the same
/// state, differing only in wording and colour.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Outcome {
    Success,
    Cancelled,
}

pub(crate) fn view(state: &FlashingFinishState, outcome: Outcome) -> Element<'_, BBImagerMessage> {
    let progress = match outcome {
        Outcome::Success => {
            let msg = if state.is_download {
                "Successfully Downloaded Image"
            } else {
                "Successfully Flashed Image"
            };
            progress_finish_view("100%", constants::CHECK_MARK_GREEN, msg)
        }
        Outcome::Cancelled => progress_finish_view(
            "Cancelled",
            constants::DANGER,
            "Flashing Cancelled by the user",
        ),
    };

    let restart = match outcome {
        Outcome::Success => button("Flash Another").style(widget::button::primary),
        Outcome::Cancelled => button("Restart").style(widget::button::danger),
    };

    page_type1(
        board_view_pane(&state.selected_board, &state.common),
        progress,
        [restart.on_press(BBImagerMessage::Restart)],
    )
}
