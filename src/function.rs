pub mod wrapper;

pub use wrapper::*;

/// See also `FunctionWrapper`.
pub trait Function: std::fmt::Display {
	/// Is input x in the definition domain.
	fn is_x_valid(&self, _x: f32) -> bool {
		true
	}

	// Evaluate the function for the input x.
	//
	// # Return
	//
	// Return `None` if x is not in the definition domain.
	fn eval(&self, x: f32) -> Option<f32>;
}
