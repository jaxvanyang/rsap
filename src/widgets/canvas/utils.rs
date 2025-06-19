use super::{EPS, UNIT};
use crate::expression::{Expression, Function};
use iced::{
	widget::canvas::{self, Stroke},
	Color, Point,
};

#[inline]
#[must_use]
pub fn is_equal(a: f32, b: f32) -> bool {
	(a - b).abs() < EPS
}

#[inline]
#[must_use]
pub fn is_zero(x: f32) -> bool {
	is_equal(x, 0.0)
}

#[inline]
pub fn cartesian_to_screen(p: &Point, center: &Point) -> Point {
	let x = center.x + p.x * UNIT;
	let y = center.y - p.y * UNIT;
	Point { x, y }
}

/// Draw background on the canvas frame.
pub fn draw_background(frame: &mut canvas::Frame) {
	let center = frame.center();
	frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);
	let w = (frame.width() / UNIT / 2.0) as i32;
	let h = (frame.height() / UNIT / 2.0) as i32;
	frame.stroke(
		&canvas::Path::line(
			Point {
				x: 0.0,
				y: center.y,
			},
			Point {
				x: frame.width(),
				y: center.y,
			},
		),
		Stroke::default().with_color(Color::from_rgb8(50, 50, 50)),
	);
	frame.stroke(
		&canvas::Path::line(
			Point {
				x: center.x,
				y: 0.0,
			},
			Point {
				x: center.x,
				y: frame.height(),
			},
		),
		Stroke::default().with_color(Color::from_rgb8(50, 50, 50)),
	);
	for i in -w..=w {
		for j in -h..=h {
			let x = center.x + i as f32 * UNIT;
			let y = center.y + j as f32 * UNIT;
			frame.fill(
				&canvas::Path::circle(Point { x, y }, 1.0),
				Color::from_rgb8(100, 100, 100),
			);
		}
	}
}

/// Draw the function on the canvas frame.
pub fn draw_function(frame: &mut canvas::Frame, expr: &Expression) {
	let center = frame.center();
	// half width in epsilons
	let w = (((frame.width() / UNIT / 2.0) as i32 + 1) as f32 / EPS) as i32;
	let mut point_groups = Vec::new();
	let mut points = Vec::new();

	for i in -w..w {
		let x = i as f32 * EPS;
		let y = if let Some(y) = expr.eval(x) {
			y
		} else {
			if !points.is_empty() {
				point_groups.push(points);
				points = Vec::new();
			}
			continue;
		};
		let point = cartesian_to_screen(&Point { x, y }, &center);
		points.push(point);
	}
	if !points.is_empty() {
		point_groups.push(points);
	}

	for points in point_groups {
		let path = canvas::Path::new(|p| {
			p.move_to(points[0]);
			for point in points.into_iter().skip(1) {
				p.line_to(point);
			}
		});
		frame.stroke(
			&path,
			Stroke::default().with_color(Color::from_rgb8(20, 200, 240)),
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cartesian_to_screen() {
		let center = Point { x: 100.0, y: 100.0 };
		let point = Point { x: 0.0, y: 0.0 };
		let screen_point = cartesian_to_screen(&point, &center);
		assert_eq!(screen_point, Point { x: 100.0, y: 100.0 });
	}
}
