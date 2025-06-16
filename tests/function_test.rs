use rsap::{f_add, f_const, f_div, f_mul, f_neg, f_sub, f_var, function::Function};

#[test]
fn function_combination() {
	// parse("-x + 1 - x * 3 / 2")
	let f = f_sub!(
		f_add!(f_neg!(f_var!()), f_const!(1)),
		f_div!(f_mul!(f_var!(), f_const!(3)), f_const!(2))
	);
	assert_eq!(f.eval(0.0).unwrap(), 1.0);
	assert_eq!(f.eval(1.0).unwrap(), -1.5);
	assert_eq!(f.eval(2.0).unwrap(), -4.0);
}

#[test]
fn function_display() {
	// parse("-x + 1 - x * 3 / 2")
	let f = f_sub!(
		f_add!(f_neg!(f_var!()), f_const!(1)),
		f_div!(f_mul!(f_var!(), f_const!(3)), f_const!(2))
	);
	assert_eq!(f.to_string(), "-x + 1 - x * 3 / 2");
}
