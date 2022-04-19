#![feature(test)]

use game_of_life::{shapes, Life, Simple};

extern crate test;

use test::Bencher;

#[bench]
fn bench_simple_blinker_step_1000(b: &mut Bencher) {
    b.iter(|| {
        let mut l = Simple::from_str(shapes::BLINKER);
        for _ in 0..1000 {
            l.step();
        }
    });
}

#[bench]
fn bench_simple_pentadecathlon_step_1000(b: &mut Bencher) {
    b.iter(|| {
        let mut l = Simple::from_str(shapes::PENTADECATHLON);
        for _ in 0..1000 {
            l.step();
        }
    });
}

#[bench]
fn bench_simple_gosper_glider_gun_step_1000(b: &mut Bencher) {
    b.iter(|| {
        let mut l = Simple::from_str(shapes::GOSPLER_GLIDER_GUN);
        for _ in 0..1000 {
            l.step();
        }
    });
}
