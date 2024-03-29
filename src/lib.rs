#![no_std]
#![warn(clippy::pedantic, missing_docs)]
#![doc = include_str!("../README.md")]
use core::{
    iter::{Copied, Cycle, Skip},
    ops::{Div, MulAssign},
    slice,
};
use derive_new::new;
use num_traits::Pow;

/// Frequency of an A at octave 0. Good base for a frequency generator.
pub const A0: f32 = 27.5;

/// Frequency generator. Acts as an iterator yielding notes from low to high pitch.
/// The number of notes in the resulting equal-tempered scale (aka TET) is adjustable.
#[derive(new, Clone)]
pub struct FreqGenerator<T = f32> {
    freq: T,
    scale: T,
}

impl<T> Iterator for FreqGenerator<T>
where
    T: From<u8> + Pow<T, Output = T> + MulAssign + Div<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.freq *= (<u8 as Into<T>>::into(2)).pow(<u8 as Into<T>>::into(1) / self.scale);
        Some(self.freq)
    }
}

/// Scale generator. Acts as an iterator yielding notes from low to high pitch.
/// Uses a [`FreqGenerator`] to know the next candidate frequency and skips these candidates if they're not part of the scale.
/// Supports modes. To change the key, shift the first note yielded by its [`FreqGenerator`] with [`skip`](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html#method.skip).
pub struct ScaleGenerator<T = f32, I = FreqGenerator<T>>
where
    I: Iterator<Item = T>,
    T: Div<Output = T> + Pow<T, Output = T> + From<u8> + MulAssign + Pow<T> + Copy,
{
    fg: I,
    intervals: Skip<Cycle<Copied<slice::Iter<'static, u8>>>>,
}

impl<T, I> ScaleGenerator<T, I>
where
    I: Iterator<Item = T>,
    T: Div<Output = T> + Pow<T, Output = T> + From<u8> + MulAssign + Pow<T> + Copy,
{
    /// Make a new generator from a [`FreqGenerator`] or similar. Any iterator wrapper works so you can use [`skip`](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html#method.skip) on it.
    /// The `mode` parametter is the shift from the current mode (C / ionian). 1 is D, 2 is E...
    ///
    /// Keep in mind that modes only work on 12 TET. Nothing here enforces this.
    pub fn new(frequencies: I, mode: usize) -> Self {
        Self {
            fg: frequencies,
            intervals: [2, 2, 1, 2, 2, 2, 1].iter().copied().cycle().skip(mode),
        }
    }
}

impl<T, I> Iterator for ScaleGenerator<T, I>
where
    I: Iterator<Item = T>,
    T: Div<Output = T> + Pow<T, Output = T> + From<u8> + MulAssign + Pow<T> + Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        for _ in 1..self.intervals.next().unwrap() {
            self.fg.next()?;
        }
        self.fg.next()
    }
}

#[cfg(test)]
mod tests {
    use super::{FreqGenerator, ScaleGenerator, A0};

    #[test]
    fn precision() {
        assert_eq!(
            440f32,
            FreqGenerator::new(A0, 12f32)
                .skip(12 * 4 - 1)
                .next()
                .unwrap()
                .round()
        );
    }

    #[test]
    fn precision_f64() {
        assert_eq!(
            440f64,
            FreqGenerator::new(A0 as f64, 12.0)
                .skip(12 * 4 - 1)
                .next()
                .unwrap()
                .round()
        );
    }

    #[test]
    fn scale_octave() {
        assert_eq!(
            440f32,
            ScaleGenerator::new(FreqGenerator::new(A0, 12f32), 0)
                .skip(4 * 7 - 1)
                .next()
                .unwrap()
                .round()
        )
    }

    #[test]
    fn major() {
        assert_eq!(
            554f32, // C#, part of A chord
            ScaleGenerator::new(FreqGenerator::new(A0, 12f32), 0)
                .skip(4 * 7 - 1 + 2)
                .next()
                .unwrap()
                .round()
        )
    }

    #[test]
    fn minor() {
        assert_eq!(
            523f32, // C#, part of A chord
            ScaleGenerator::new(FreqGenerator::new(A0, 12f32), 5)
                .skip(4 * 7 - 1 + 2)
                .next()
                .unwrap()
                .round()
        )
    }

    #[test]
    fn e_major() {
        assert_eq!(
            740f32, // F#, part of E chord
            ScaleGenerator::new(FreqGenerator::new(A0, 12f32).skip(5), 0)
                .skip(4 * 7 + 1)
                .next()
                .unwrap()
                .round()
        )
    }
}
