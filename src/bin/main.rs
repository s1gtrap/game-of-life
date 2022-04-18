#![feature(stdin_forwarders)]
#![allow(dead_code)]

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
    use game_of_life::Life;
    let mut life = <game_of_life::Simple as Life>::from(PULSAR);
    println!("{}", life.display());
    while let Some(_) = std::io::stdin().lines().next() {
        life.step();
        println!("{}", life.display());
    }
}
