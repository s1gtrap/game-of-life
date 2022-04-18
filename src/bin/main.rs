#![feature(stdin_forwarders)]

fn main() {
    let w = 3;
    let h = 3;
    let mut counts = vec![0; (w + 2) * (h + 2)];
    let mut states: Vec<_> = r#"     
  .  
  .  
  .  
     
"#
    .lines()
    .map(|l| l.chars())
    .flatten()
    .map(|c| c == '.')
    .collect();
    let print = |states: &[bool]| {
        println!("+{:-<1$}+", "", w + 2);
        for i in 0..(h + 2) {
            print!("|");
            for j in 0..(w + 2) {
                if states[i * (w + 2) + j] {
                    print!("w");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
        println!("+{:-<1$}+", "", w + 2);
    };
    print(&states);
    while let Some(_) = std::io::stdin().lines().next() {
        game_of_life::gameoflife(&mut counts, &mut states, w, h);
        print(&states);
    }
}
