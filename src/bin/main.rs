#![feature(stdin_forwarders)]

const BLINKER: (usize, usize, &'static str) = (
    3,
    3,
    r#"     
  .  
  .  
  .  
     "#,
);
const TOAD: (usize, usize, &'static str) = (
    4,
    4,
    r#"      
   .  
 .  . 
 .  . 
  .   
      
   "#,
);
const BEACON: (usize, usize, &'static str) = (
    4,
    4,
    r#"      
 ..   
 .    
    . 
   .. 
      
   "#,
);
const PULSAR: (usize, usize, &'static str) = (
    15,
    15,
    r#"                 
                 
    ...   ...    
                 
  .    . .    .  
  .    . .    .  
  .    . .    .  
    ...   ...    
                 
    ...   ...    
  .    . .    .  
  .    . .    .  
  .    . .    .  
                 
    ...   ...    
                 
                 "#,
);

fn main() {
    let (w, h, shape) = PULSAR;
    let mut counts = vec![0; (w + 2) * (h + 2)];
    let mut states: Vec<_> = shape
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
