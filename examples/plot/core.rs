pub mod message;
pub mod state;

use iced::{
	alignment::{Horizontal, Vertical},
	widget::{canvas, container, row, stack, text, text_input},
	Element,
	Length::{self, Fill},
};
pub use message::*;
use rsap::{f_const, f_div, f_var, function::FunctionWrapper, widgets::empty_canvas};
pub use state::*;

pub fn update(state: &mut State, message: Message) {
	match message {
		Message::InputChanged(input) => {
			// TODO: parse input to function
			state.function = if input.is_empty() {
				None
			} else {
				Some(FunctionWrapper::new(f_div!(f_const!(1), f_var!())))
			};

			state.input = input;
		}
	}
}

pub fn view(state: &State) -> Element<Message> {
	let input: Element<_> = container(
		row!(
			container(text!("y = ")).padding(5),
			text_input("Type function expression", &state.input)
				.on_input(Message::InputChanged)
				.width(500),
		)
		.padding(10),
	)
	.width(Length::Fill)
	.height(Length::Fill)
	.align_x(Horizontal::Right)
	.align_y(Vertical::Bottom)
	.into();

	let plot_canvas: Element<_> = if let Some(f) = &state.function {
		canvas(f.clone()).width(Fill).height(Fill).into()
	} else {
		empty_canvas().width(Fill).height(Fill).into()
	};

	let content = stack!(plot_canvas, input);

	content.into()
}
