mod core;

use core::{State, update, view};
use iced::Theme;

fn main() -> iced::Result {
	iced::application("Plot", update, view)
		.theme(theme)
		.run_with(|| (State::default(), iced::widget::text_input::focus("input")))
}

fn theme(_state: &State) -> Theme {
	Theme::TokyoNight
}
