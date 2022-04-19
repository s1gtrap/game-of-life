use crate::Life;

pub struct Simple(usize, usize, Vec<u8>, Vec<bool>);

impl Life for Simple {
    fn new<I>(w: usize, h: usize, i: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Simple(w, h, vec![0; (w + 2) * (h + 2)], i.into_iter().collect())
    }

    fn width(&self) -> usize {
        self.0 + 2
    }

    fn height(&self) -> usize {
        self.1 + 2
    }

    fn cells(&self) -> &[bool] {
        &self.3[..]
    }

    fn step(&mut self) {
        gameoflife(&mut self.2, &mut self.3, self.0, self.1);
    }
}

fn gameoflife(counts: &mut [u8], states: &mut [bool], width: usize, height: usize) {
    countcounts(counts, &*states, width, height);
    for i in 0..height {
        for j in 0..width {
            let coord = (i + 1) * (width + 2) + (j + 1);
            let currentvalue = states[coord];
            let neighbors = counts[(i + 1) * (width + 2) + j + 1];
            if currentvalue {
                if neighbors < 2 || neighbors > 3 {
                    states[coord] = false;
                }
            } else {
                if neighbors == 3 {
                    states[coord] = true;
                }
            }
        }
    }
}

fn countcounts(counts: &mut [u8], states: &[bool], width: usize, height: usize) {
    counts.iter_mut().for_each(|x| *x = 0);
    let awidth = width + 2;
    for i in 0..height {
        for j in 0..width {
            let val = states[(i + 1) * awidth + (j + 1)] as u8;
            counts[(i + 1 + 1) * awidth + j + 1] += val;
            counts[(i + 1 - 1) * awidth + j + 1] += val;
            counts[(i + 1) * awidth + j + 1 + 1] += val;
            counts[(i + 1) * awidth + j - 1 + 1] += val;
            counts[(i + 1 + 1) * awidth + (j + 1) + 1] += val;
            counts[(i + 1 - 1) * awidth + (j + 1) - 1] += val;
            counts[(i + 1 - 1) * awidth + j + 1 + 1] += val;
            counts[(i + 1 + 1) * awidth + (j + 1) - 1] += val;
        }
    }
}

#[test]
fn test_simple_step() {
    let mut life = <Simple as Life>::from_str(super::shapes::BLINKER);
    life.step();
    assert_eq!(
        life.cells(),
        Simple::from_str(
            "     
     
 ... 
     
     "
        )
        .cells(),
    );
    life.step();
    assert_eq!(
        life.cells(),
        Simple::from_str(
            "     
  .  
  .  
  .  
     "
        )
        .cells(),
    );

    let mut life = <Simple as Life>::from_str(super::shapes::TOAD);
    life.step();
    assert_eq!(
        life.cells(),
        Simple::from_str(
            "      
      
  ... 
 ...  
      
      "
        )
        .cells(),
    );
    life.step();
    assert_eq!(
        life.cells(),
        Simple::from_str(
            "      
   .  
 .  . 
 .  . 
  .   
      "
        )
        .cells(),
    );
}
