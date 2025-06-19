use rsap::{add, expression::Function, mul, neg, num, parse, var};

#[test]
fn simple_expression() {
	// -x + 1 * 2
	let f = add!(neg!(var!()), mul!(num!(1.0), num!(2.0)));
	assert_eq!(f.eval(0.0).unwrap(), 2.0);
	assert_eq!(f.eval(1.0).unwrap(), 1.0);
}

#[test]
fn invalid_expression() {
	assert!(parse!("x x").is_err());
	assert!(parse!("-").is_err());
}
