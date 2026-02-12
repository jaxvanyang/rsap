mod core;

use core::{update, view, State};
use iced::Theme;

fn main() -> iced::Result {
	iced::application(
		|| (State::default(), iced::widget::operation::focus("input")),
		update,
		view,
	)
	.theme(theme)
	.antialiasing(true)
	.title("Plot")
	.run()
}

fn theme(_state: &State) -> Theme {
	Theme::TokyoNight
}
