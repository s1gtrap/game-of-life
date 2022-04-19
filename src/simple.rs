use crate::Life;

pub struct Simple {
    width: usize,
    height: usize,
    counts: Vec<u8>,
    states: Vec<bool>,
}

impl Life for Simple {
    fn new<I>(width: usize, height: usize, i: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Simple {
            width,
            height,
            counts: vec![0; (width + 2) * (height + 2)],
            states: i.into_iter().collect(),
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn cells(&self) -> &[bool] {
        &self.states[..]
    }

    fn step(&mut self) {
        // reset counts
        self.counts.iter_mut().for_each(|x| *x = 0);

        // count
        let awidth = self.width;
        for i in 0..self.height {
            for j in 0..self.width {
                let val = self.states[i * awidth + j] as u8;
                self.counts[(i + 1 + 1) * awidth + j + 1] += val;
                self.counts[(i + 1 - 1) * awidth + j + 1] += val;
                self.counts[(i + 1) * awidth + j + 1 + 1] += val;
                self.counts[(i + 1) * awidth + j - 1 + 1] += val;
                self.counts[(i + 1 + 1) * awidth + (j + 1) + 1] += val;
                self.counts[(i + 1 - 1) * awidth + (j + 1) - 1] += val;
                self.counts[(i + 1 - 1) * awidth + j + 1 + 1] += val;
                self.counts[(i + 1 + 1) * awidth + (j + 1) - 1] += val;
            }
        }

        // iterate
        for i in 0..self.height {
            for j in 0..self.width {
                let coord = i * (self.width) + j;
                let currentvalue = self.states[coord];
                let neighbors = self.counts[(i + 1) * (self.width) + j + 1];
                if currentvalue {
                    if neighbors < 2 || neighbors > 3 {
                        self.states[coord] = false;
                    }
                } else {
                    if neighbors == 3 {
                        self.states[coord] = true;
                    }
                }
            }
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
