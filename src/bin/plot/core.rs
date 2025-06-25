pub mod message;
pub mod state;

use iced::{
	alignment::{Horizontal, Vertical},
	widget::{canvas, container, row, stack, text, text_input},
	Element,
	Length::{self, Fill},
};
pub use message::*;
use rsap::{expression::parse, widgets::empty_canvas};
pub use state::*;

pub fn update(state: &mut State, message: Message) {
	match message {
		Message::InputChanged(input) => {
			state.expression = parse(&input).ok();
			state.input = input;
		}
	}
}

pub fn view(state: &State) -> Element<Message> {
	let mut input = text_input("Type function expression", &state.input)
		.id("input")
		.on_input(Message::InputChanged)
		.width(500);

	if !state.input.is_empty() && state.expression.is_none() {
		input = input.icon(text_input::Icon {
			font: iced::Font::default(),
			code_point: '⚠',
			size: Some(iced::Pixels(18.0)),
			spacing: 10.0,
			side: text_input::Side::Right,
		});
	}

	let input_row: Element<_> =
		container(row!(container(text!("y = ")).padding(5), input,).padding(10))
			.width(Length::Fill)
			.height(Length::Fill)
			.align_x(Horizontal::Right)
			.align_y(Vertical::Bottom)
			.into();

	let plot_canvas: Element<_> = if let Some(expr) = &state.expression {
		canvas(expr.clone()).width(Fill).height(Fill).into()
	} else {
		empty_canvas().width(Fill).height(Fill).into()
	};

	let content = stack!(plot_canvas, input_row);

	content.into()
}
