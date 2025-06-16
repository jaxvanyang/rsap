pub mod utils;

use crate::function::FunctionWrapper;
use iced::{
	mouse,
	widget::canvas::{self, Program},
	Renderer, Theme,
};
pub use utils::*;

/// pixels per unit
pub const UNIT: f32 = 50.0;

impl<Message> Program<Message> for FunctionWrapper {
	type State = ();

	fn draw(
		&self,
		_state: &Self::State,
		renderer: &Renderer,
		_theme: &Theme,
		bounds: iced::Rectangle,
		_cursor: mouse::Cursor,
	) -> Vec<canvas::Geometry> {
		let mut frame = canvas::Frame::new(renderer, bounds.size());

		draw_background(&mut frame);
		draw_function(&mut frame, self);

		vec![frame.into_geometry()]
	}
}

pub struct EmptyCanvas;

impl<Message> Program<Message> for EmptyCanvas {
	type State = ();

	fn draw(
		&self,
		_state: &Self::State,
		renderer: &Renderer,
		_theme: &Theme,
		bounds: iced::Rectangle,
		_cursor: mouse::Cursor,
	) -> Vec<canvas::Geometry> {
		let mut frame = canvas::Frame::new(renderer, bounds.size());

		draw_background(&mut frame);

		vec![frame.into_geometry()]
	}
}

/// Create an empty canvas with background.
pub fn empty_canvas<Message>() -> canvas::Canvas<EmptyCanvas, Message> {
	canvas::Canvas::new(EmptyCanvas)
}
