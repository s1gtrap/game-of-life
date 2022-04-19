#![feature(stdin_forwarders)]
#![allow(dead_code)]

fn main() {
    use game_of_life::Life;
    let mut life = <game_of_life::Simple as Life>::from_str(game_of_life::shapes::TOAD_S);
    println!("{}", life.display());
    while let Some(_) = std::io::stdin().lines().next() {
        life.step();
        println!("{}", life.display());
    }
}
