#[macro_export]
macro_rules! func2 {
	($f_name:expr, $lhs:expr, $rhs:expr) => {
		$crate::expression::Func2::new($f_name, $lhs, $rhs)
	};
}

#[macro_export]
macro_rules! log {
	($lhs:expr, $rhs:expr) => {
		$crate::func2!("log", $lhs, $rhs).unwrap()
	};
}
