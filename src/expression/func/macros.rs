#[macro_export]
macro_rules! func {
	($f_name:expr, $e:expr) => {
		$crate::expression::Func::new($f_name, $e)
	};
}

#[macro_export]
macro_rules! sin {
	($e:expr) => {
		$crate::func!("sin", $e).unwrap()
	};
}

#[macro_export]
macro_rules! cos {
	($e:expr) => {
		$crate::func!("cos", $e).unwrap()
	};
}

#[macro_export]
macro_rules! tan {
	($e:expr) => {
		$crate::func!("tan", $e).unwrap()
	};
}

#[macro_export]
macro_rules! cot {
	($e:expr) => {
		$crate::func!("cot", $e).unwrap()
	};
}

#[macro_export]
macro_rules! sec {
	($e:expr) => {
		$crate::func!("sec", $e).unwrap()
	};
}

#[macro_export]
macro_rules! csc {
	($e:expr) => {
		$crate::func!("csc", $e).unwrap()
	};
}

#[macro_export]
macro_rules! arcsin {
	($e:expr) => {
		$crate::func!("arcsin", $e).unwrap()
	};
}

#[macro_export]
macro_rules! arccos {
	($e:expr) => {
		$crate::func!("arccos", $e).unwrap()
	};
}

#[macro_export]
macro_rules! arctan {
	($e:expr) => {
		$crate::func!("arctan", $e).unwrap()
	};
}

#[macro_export]
macro_rules! arccot {
	($e:expr) => {
		$crate::func!("arccot", $e).unwrap()
	};
}

#[macro_export]
macro_rules! ln {
	($e:expr) => {
		$crate::func!("ln", $e).unwrap()
	};
}

#[macro_export]
macro_rules! sqrt {
	($e:expr) => {
		$crate::func!("sqrt", $e).unwrap()
	};
}
