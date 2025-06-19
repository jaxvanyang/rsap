#[macro_export]
macro_rules! unary_expr {
	($op:expr, $e:expr) => {
		$crate::expression::Unary::new($op, $e)
	};
}

#[macro_export]
macro_rules! neg {
	($e:expr) => {
		$crate::unary_expr!("-", $e).unwrap()
	};
}
