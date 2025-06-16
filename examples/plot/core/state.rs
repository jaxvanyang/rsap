use rsap::function::FunctionWrapper;

#[derive(Default)]
pub struct State {
	pub function: Option<FunctionWrapper>,
	pub input: String,
}
