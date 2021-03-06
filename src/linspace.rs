use super::misc::ToFloat;
use std::ops::{Add, Sub, Div};

/// An iterator of a sequence of evenly spaced floats.
///
/// Iterator element type is `F`.
pub struct Linspace<F> {
    start: F,
    step: F,
    len: usize,
}

impl<F> Iterator for Linspace<F> where
    F: Copy + Add<Output=F>,
{
    type Item = F;

    #[inline]
    fn next(&mut self) -> Option<F>
    {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elt = self.start;
            self.start = self.start + self.step;
            Some(elt)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        (self.len, Some(self.len))
    }
}

impl<F> ExactSizeIterator for Linspace<F> where
    Linspace<F>: Iterator
{ }

/// Return an iterator with `n` elements, where the first
/// element is `a` and the last element is `b`.
///
/// Iterator element type is `F`.
///
/// ```
/// use itertools::linspace;
///
/// itertools::assert_equal(linspace::<f32>(0., 1., 5),
///                         vec![0., 0.25, 0.5, 0.75, 1.0]);
/// ```
#[inline]
pub fn linspace<F>(a: F, b: F, n: usize) -> Linspace<F> where
    F: Copy + Sub<Output=F> + Div<Output=F>,
    usize: ToFloat<F>,
{
    let nf: F = n.to_float();
    let step = (b - a)/(nf - 1usize.to_float());
    Linspace {
        start: a,
        step: step,
        len: n,
    }
}
