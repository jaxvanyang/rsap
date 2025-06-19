pub trait Function {
	/// Is input x in the definition domain.
	#[allow(unused_variables)]
	fn is_x_valid(&self, x: f32) -> bool {
		true
	}

	// Evaluate the function for the input x.
	//
	// # Return
	//
	// Return `None` if x is not in the definition domain.
	fn eval(&self, x: f32) -> Option<f32>;
}
