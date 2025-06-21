use rsap::expression::{parse, Function};

#[test]
fn simple_expression() {
	// -x + 1 * 2
	let f = parse("-x + 1 * 2").unwrap();
	assert_eq!(f.eval(0.0).unwrap(), 2.0);
	assert_eq!(f.eval(1.0).unwrap(), 1.0);
}

#[test]
fn invalid_expression() {
	assert!(parse("x x").is_err());
	assert!(parse("-").is_err());
}
