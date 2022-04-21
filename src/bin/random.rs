#![feature(div_duration, stdin_forwarders)]

use std::time::{Duration, Instant};

fn main() {
    use rand::Rng;
    use std::io;
    use std::io::Write;

    env_logger::init();

    let n: usize = std::env::args()
        .nth(1)
        .expect("specify size")
        .parse()
        .unwrap();
    let mut rng = rand::thread_rng();
    let mut states = (0..n * n)
        .map(|_| [0, 0, 0, rng.gen_range(0..=1u8) * 255])
        .flatten()
        .collect::<Vec<_>>();
    let mut life = game_of_life::init(n, n, &states);
    //log::debug!("{}", life.display());
    let start = Instant::now();
    for _ in 0..100 {
        print!(".");
        io::stdout().flush().unwrap();

        game_of_life::step(&mut life, &mut states);
        //log::debug!("{}", life.display());
    }
    let duration = start.elapsed();
    println!("time per iteration {:?}", duration.div_f32(100.0));
}
