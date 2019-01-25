#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use draw::{self, samples, Drawable, Result, Sprite};
use rand::{self, Rng};

fn draw_benchmark(c: &mut Criterion) {
    let rectangles = samples::many_rectangles(10000);
    let mut rng = rand::thread_rng();

    c.bench_function("get_pixel", move |b| {
        b.iter(|| {
            let x = rng.gen_range(0, rectangles.width());
            let y = rng.gen_range(0, rectangles.height());
            rectangles.get_pixel(x, y);
        })
    });

    let mut container = Sprite::new_container(500, 500, 4);

    c.bench_function("add_child", move |b| {
        b.iter(|| container.add_child(Sprite::new(100, 100)))
    });
}

criterion_group!(benches, draw_benchmark);
criterion_main!(benches);
