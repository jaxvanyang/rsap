// A minimal ray-tracer.
//
// Usage: `minitracer [outfile]`

use rand::prelude::*;
use rand::rngs::SmallRng;
use rsap::*;
use std::env;
use std::f32;
use std::f32::consts::FRAC_1_SQRT_2;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn random_point(rng: &mut (impl Rng + SeedableRng)) -> Vec3f {
	let limit = 5.0;
	Vec3f::new(
		rng.gen_range(-limit..=limit),
		rng.gen_range(-limit..=limit),
		rng.gen_range(-limit..=limit),
	)
}

fn random_color(rng: &mut (impl Rng + SeedableRng)) -> Vec3f {
	Vec3f::new(rng.gen(), rng.gen(), rng.gen())
}

fn random_sphere(rng: &mut (impl Rng + SeedableRng)) -> Sphere {
	Sphere::new(
		random_point(rng),
		rng.gen_range(0.5..=2.0),
		random_color(rng),
	)
}

fn random_disk(rng: &mut (impl Rng + SeedableRng)) -> Disk {
	Disk::new(
		random_point(rng),
		random_point(rng).normalized(),
		rng.gen_range(0.5..=2.0),
		random_color(rng),
	)
}

fn random_aabb(rng: &mut (impl Rng + SeedableRng)) -> AABB {
	let min = random_point(rng);
	let size = Vec3f::new(
		rng.gen_range(0.5..2.0),
		rng.gen_range(0.5..2.0),
		rng.gen_range(0.5..2.0),
	);
	let max = min + size;

	AABB::new(min, max, random_color(rng))
}

fn main() {
	let outfile = env::args().nth(1).unwrap_or(String::from("out.ppm"));

	// set options
	let option = RenderOpt::new(
		800,
		600,
		120.0,
		Matrix44f([
			[FRAC_1_SQRT_2, -0.5, 0.5, 5.0],
			[-FRAC_1_SQRT_2, -0.5, 0.5, 5.0],
			[0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 5.0],
			[0.0, 0.0, 0.0, 1.0],
		]),
	);

	let mut rng = SmallRng::seed_from_u64(0);

	// add objects
	let mut objects: Vec<Box<dyn Object>> = Vec::new();
	objects.push(Box::new(Plane::new(
		Vec3f::new(0.0, 0.0, -5.0),
		Vec3f::new(0.0, 0.0, -1.0),
		random_color(&mut rng),
	)));
	for _ in 0..10 {
		let sphere = random_sphere(&mut rng);
		println!("{sphere:?}");
		objects.push(Box::new(sphere));
	}
	for _ in 0..10 {
		let disk = random_disk(&mut rng);
		println!("{disk:?}");
		objects.push(Box::new(disk));
	}
	objects.push(Box::new(AABB::new(
		Vec3f::sames(-1.0),
		Vec3f::sames(1.0),
		random_color(&mut rng),
	)));
	for _ in 0..10 {
		let aabb = random_aabb(&mut rng);
		println!("{aabb:?}");
		objects.push(Box::new(aabb));
	}

	// render
	let image = render(&option, &objects);

	let outfile = Path::new(&outfile);
	let mut outfile = File::create(&outfile).unwrap();

	outfile.write_all(&image.to_ppm()).unwrap();
}

fn render(option: &RenderOpt, objects: &Vec<Box<dyn Object>>) -> Image {
	let mut image = Image::new(option.width, option.height);
	let scale = option.angle();
	let orig = option.camera_to_world.mul_point(&Vec3f::zeros());

	for i in 0..option.height {
		for j in 0..option.width {
			let x = (j as f32 + 0.5) / option.width as f32 * 2.0 - 1.0;
			let y = 1.0 - (i as f32 + 0.5) / option.height as f32 * 2.0;
			let x = x * scale * option.aspect_ratio();
			let y = y * scale;

			let dir = option
				.camera_to_world
				.mul_vector(&Vec3f::new(x, y, -1.0))
				.normalized();

			image[i * option.width + j] = cast_ray(&orig, &dir, &objects);
		}
	}

	image
}

/// Return a pair of the closest intersection distance and the intersected object.
fn trace<'a>(
	orig: &Vec3f,
	dir: &Vec3f,
	objects: &'a Vec<Box<dyn Object>>,
) -> Option<(f32, &'a Box<dyn Object>)> {
	let mut tnear: Option<f32> = None;
	let mut hit_obj: Option<&Box<dyn Object>> = None;

	for obj in objects {
		if let Some(t) = obj.intersect(orig, dir) {
			if tnear.is_none() {
				tnear = Some(t);
				hit_obj = Some(obj);
			} else if t < tnear.unwrap() {
				tnear = Some(t);
				hit_obj = Some(obj);
			}
		}
	}

	if tnear.is_none() {
		None
	} else {
		Some((tnear.unwrap(), hit_obj.unwrap()))
	}
}

/// Compute the color at the intersection point if any (return background color otherwise)
fn cast_ray(orig: &Vec3f, dir: &Vec3f, objects: &Vec<Box<dyn Object>>) -> Vec3f {
	if let Some((t, hit_obj)) = trace(orig, dir, objects) {
		let phit = *orig + *dir * t;
		return hit_obj.shade(dir, &phit);
	}

	Vec3f::zeros()
}
