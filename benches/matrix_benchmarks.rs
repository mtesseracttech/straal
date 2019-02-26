#[macro_use]
extern crate criterion;

use criterion::Criterion;

use straal::*;

fn mat3_mul(c: &mut Criterion) {
    c.bench_function("mat3 multiplication", |b| b.iter(|| Mat3::identity() * Mat3::identity()));
}

fn zxy_angle_axes_bench(c: &mut Criterion) {
    c.bench_function("optimized mat3 angles to axes zxy", |b| b.iter(|| Mat3::angles_to_axes_zxy(Vec3::new(89.0, 89.0, 89.0))));
}

criterion_group!(benches, zxy_angle_axes_bench);
criterion_main!(benches);