use super::Function;
use std::{ops::Deref, rc::Rc};

/// Wrapper sturct for `Function` trait.
#[derive(Clone)]
pub struct FunctionWrapper(Rc<dyn Function>);

impl FunctionWrapper {
	pub fn new(f: impl Into<Rc<dyn Function>>) -> Self {
		Self(f.into())
	}
}

impl Deref for FunctionWrapper {
	type Target = dyn Function;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}
