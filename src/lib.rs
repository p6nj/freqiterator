use num_traits::Pow;
use std::ops::{Div, MulAssign};

const START_FREQ: f32 = 13.75;

pub struct FreqIterator<T = f32> {
    freq: T,
    scale: T,
}

impl<T> FreqIterator<T>
where
    T: From<f32> + From<u8>,
{
    pub fn new() -> Self {
        FreqIterator {
            freq: START_FREQ.into(),
            scale: 12.into(),
        }
    }

    pub fn with_scale(scale: u8) -> Self {
        FreqIterator {
            freq: START_FREQ.into(),
            scale: scale.into(),
        }
    }
}

impl<T> Iterator for FreqIterator<T>
where
    T: From<u8> + Pow<T, Output = T> + MulAssign + Div<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.freq *= (<u8 as Into<T>>::into(2)).pow(<u8 as Into<T>>::into(1) / self.scale);
        Some(self.freq)
    }
}
