#[macro_export]
macro_rules! binary_expr {
	($op:expr, $lhs:expr, $rhs:expr) => {
		$crate::expression::Binary::new($op, $lhs, $rhs)
	};
}

#[macro_export]
macro_rules! add {
	($lhs:expr, $rhs:expr) => {
		$crate::binary_expr!("+", $lhs, $rhs).unwrap()
	};
}

#[macro_export]
macro_rules! sub {
	($lhs:expr, $rhs:expr) => {
		$crate::binary_expr!("-", $lhs, $rhs).unwrap()
	};
}

#[macro_export]
macro_rules! mul {
	($lhs:expr, $rhs:expr) => {
		$crate::binary_expr!("*", $lhs, $rhs).unwrap()
	};
}

#[macro_export]
macro_rules! div {
	($lhs:expr, $rhs:expr) => {
		$crate::binary_expr!("/", $lhs, $rhs).unwrap()
	};
}

#[macro_export]
macro_rules! pow {
	($lhs:expr, $rhs:expr) => {
		$crate::binary_expr!("**", $lhs, $rhs).unwrap()
	};
}
