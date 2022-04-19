use std::fmt;

pub mod shapes;
mod simple;

pub use simple::Simple;

pub trait Life: Sized {
    fn new<I>(width: usize, height: usize, i: I) -> Self
    where
        I: IntoIterator<Item = bool>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn cells(&self) -> &[bool];
    fn step(&mut self);

    fn from_str(s: &'static str) -> Self {
        let w = s.lines().next().unwrap().len();
        let h = s.lines().count();
        println!("{},{}", w, h);
        Self::new(
            w - 2,
            h - 2,
            s.lines()
                .enumerate()
                .map(|(i, l)| {
                    assert_eq!(w, l.len(), "{}", i);
                    l.chars()
                })
                .flatten()
                .map(|c| c == '.'),
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
        writeln!(f, "+{:-<1$}+", "", self.0.width())?;
        for i in 0..(self.0.height()) {
            write!(f, "|")?;
            for j in 0..(self.0.width()) {
                if self.0.cells()[i * (self.0.width()) + j] {
                    write!(f, "w")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+{:-<1$}+", "", self.0.width())?;
        Ok(())
    }
}

#[test]
fn test_life_from_str() {
    let blinker = Simple::from_str(shapes::BLINKER);
    assert_eq!(blinker.width(), 5);
    assert_eq!(blinker.height(), 5);
    assert_eq!(
        blinker.cells(),
        &[
            false, false, false, false, false, false, false, true, false, false, false, false,
            true, false, false, false, false, true, false, false, false, false, false, false,
            false,
        ][..],
    );
    let toad = Simple::from_str(shapes::TOAD);
    assert_eq!(toad.width(), 6);
    assert_eq!(toad.height(), 6);
    assert_eq!(
        toad.cells(),
        &[
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false,
        ][..],
    );
}
