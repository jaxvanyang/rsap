mod core;

use core::{update, view, State};
use iced::Theme;

fn main() -> iced::Result {
	iced::application("Plot", update, view)
		.theme(theme)
		.antialiasing(true)
		.run_with(|| (State::default(), iced::widget::text_input::focus("input")))
}

fn theme(_state: &State) -> Theme {
	Theme::TokyoNight
}
