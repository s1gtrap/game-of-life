#![feature(div_duration, stdin_forwarders)]

use std::time::{Duration, Instant};

fn main() {
    use std::io::{self, Write};

    use game_of_life::Life;
    use rand::Rng;

    env_logger::init();

    let n: usize = std::env::args()
        .nth(1)
        .expect("specify size")
        .parse()
        .unwrap();
    let mut rng = rand::thread_rng();
    let mut life = game_of_life::Simple::new(n, n, (0..n * n).map(|_| rng.gen::<bool>()));
    //log::debug!("{}", life.display());
    let start = Instant::now();
    for _ in 0..100 {
        print!(".");
        io::stdout().flush().unwrap();

        life.step();
        //log::debug!("{}", life.display());
    }
    let duration = start.elapsed();
    println!("time per iteration {:?}", duration.div_f32(100.0));
}
