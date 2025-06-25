use rsap::expression::Expression;

#[derive(Default)]
pub struct State {
	pub expression: Option<Expression>,
	pub input: String,
}
