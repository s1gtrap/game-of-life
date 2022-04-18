use std::fmt;

pub mod shapes;

pub trait Life: Sized {
    fn new<I>(width: usize, height: usize, i: I) -> Self
    where
        I: IntoIterator<Item = bool>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn cells(&self) -> &[bool];
    fn step(&mut self);

    fn from((w, h, s): (usize, usize, &'static str)) -> Self {
        Self::new(
            w,
            h,
            s.lines().map(|l| l.chars()).flatten().map(|c| c == '.'),
        )
    }

    fn display(&self) -> Display<'_, Self> {
        Display(self)
    }
}

pub struct Display<'a, L>(&'a L)
where
    L: Life;

impl<'a, L> fmt::Display for Display<'a, L>
where
    L: Life,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+{:-<1$}+", "", self.0.width() + 2)?;
        for i in 0..(self.0.height() + 2) {
            write!(f, "|")?;
            for j in 0..(self.0.width() + 2) {
                if self.0.cells()[i * (self.0.width() + 2) + j] {
                    write!(f, "w")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+{:-<1$}+", "", self.0.width() + 2)?;
        Ok(())
    }
}

pub struct Simple(usize, usize, Vec<u8>, Vec<bool>);

impl Life for Simple {
    fn new<I>(w: usize, h: usize, i: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Simple(w, h, vec![0; (w + 2) * (h + 2)], i.into_iter().collect())
    }

    fn width(&self) -> usize {
        self.0
    }

    fn height(&self) -> usize {
        self.1
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
fn test_countcounts() {
    let w = 3;
    let h = 3;
    let mut counts = vec![0; (w + 2) * (h + 2)];
    let mut states = vec![
        false, false, false, false, false, false, false, true, false, false, false, false, true,
        false, false, false, false, true, false, false, false, false, false, false, false,
    ];
    countcounts(&mut counts, &states, 3, 3);
    assert_eq!(
        counts,
        vec![0, 1, 1, 1, 0, 0, 2, 1, 2, 0, 0, 3, 2, 3, 0, 0, 2, 1, 2, 0, 0, 1, 1, 1, 0,]
    )
}
