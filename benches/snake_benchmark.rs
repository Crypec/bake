use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Copy, Clone)]
pub struct Position {
	pub x: f32,
	pub y: f32,
}

pub fn dist_pow(lhs: Position, rhs: Position) -> f32 {
	let dx = rhs.x - lhs.x;
	let dy = rhs.y - lhs.y;
	f32::sqrt(dy.powi(2) + dx.powi(2))
}

pub fn dist_mad(lhs: Position, rhs: Position) -> f32 {
	let dx = rhs.x - lhs.x;
	let dy = rhs.y - lhs.y;
	f32::sqrt(dy.mul_add(dy, dx * dx))
}

fn dist_pow_bench(c: &mut Criterion) {
	let lhs = Position {
		x: 128.23,
		y: 2393.05,
	};
	let rhs = Position {
		x: 10.73,
		y: 54.88,
	};
	c.bench_function("dist_pow ", |b| b.iter(|| dist_pow(black_box(lhs), black_box(rhs))));
}

fn dist_mad_bench(c: &mut Criterion) {
	let lhs = Position {
		x: 128.23,
		y: 2393.05,
	};
	let rhs = Position {
		x: 10.73,
		y: 54.88,
	};
	c.bench_function("dist_pow ", |b| b.iter(|| dist_mad(black_box(lhs), black_box(rhs))));
}

criterion_group!(benches, dist_pow_bench, dist_mad_bench);
criterion_main!(benches);
