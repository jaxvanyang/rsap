mod core;

use core::{update, view, State};
use iced::Theme;

fn main() -> iced::Result {
	iced::application("Plot", update, view).theme(theme).run()
}

fn theme(_state: &State) -> Theme {
	Theme::TokyoNight
}
