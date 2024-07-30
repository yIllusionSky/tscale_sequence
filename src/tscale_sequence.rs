//! Recursiver is a library for computing recursive sequences.
use std::{
    fmt::Debug,
    mem::ManuallyDrop,
    ops::{Add, Div, Mul},
};

const DEFAULT_GEN_LEN: usize = 10000;

pub struct TScale<T, const C: usize> {
    array: [T; C],
    weight: [T; C],
}

impl<const C: usize> Default for TScale<f64, C> {
    fn default() -> Self {
        assert!(C > 0, "C must be greater than 0");
        Self {
            array: [1.0; C],
            weight: [1.0; C],
        }
    }
}

impl<T, const C: usize> TScale<T, C>
where
    Self: Default,
{
    /// create new recursive
    pub fn new() -> Self {
        Self::default()
    }

    /// create new recursive with len
    /// 
    /// The array array represents the initial values and is in ascending order, 
    /// but the weights, represented by weight, are based on the most recent state, 
    /// so the weight array is in descending order.
    pub const fn new_with_config(array: [T; C], weight: [T; C]) -> Self {
        Self {
            array,
            weight
        }
    }

    // BUG! this function reverse not work
    // If fix the bug, in iterator, you array must be front not reverse
    // pub fn new_with_config(mut array: [T; C], weight: [T; C]) -> Self {
    //     array.reverse();
    //     Self {
    //         array,
    //         weight
    //     }
    // }

    /// into iterator
    pub fn iter(&mut self) -> TScaleIter<'_, T, C> {
        let Self {
            array,
            weight,
        } = self;

        TScaleIter {
            array,
            weight,
            gen_len: DEFAULT_GEN_LEN,
        }
    }
}

/// Recursive iterator
pub struct TScaleIter<'a, T, const C: usize> {
    array: &'a mut [T; C],
    weight: &'a mut [T; C],
    gen_len: usize,
}

impl<'a, T, const C: usize> Iterator for TScaleIter<'a, T, C>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.gen_len > 0 {
            self.gen_len -= 1;

            let last_value = self
                .array
                .iter()
                .zip(self.weight.iter().rev())
                .fold(T::default(), |acc, (a, b)| acc + a.clone() * b.clone());
            let first_value = self.array[0].clone();
            (0..C - 1).for_each(|index| self.array[index] = self.array[index + 1].clone());
            self.array[C - 1] = last_value;
            Some(first_value)
        } else {
            None
        }
    }
}

impl<'a, T, const C: usize> IntoIterator for &'a mut TScale<T, C>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Default,
{
    type Item = T;
    type IntoIter = TScaleIter<'a, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        let TScale {
            array,
            weight,
        } = self;
        TScaleIter {
            array,
            weight,
            gen_len: DEFAULT_GEN_LEN,
        }
    }
}

pub struct TScaleIntoIter<T, const C: usize> {
    array: ManuallyDrop<[T; C]>,
    weight: ManuallyDrop<[T; C]>,
    gen_len: usize,
}
impl<T, const C: usize> Drop for TScaleIntoIter<T, C> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.array);
            ManuallyDrop::drop(&mut self.weight);
        }
    }
}

impl<T, const C: usize> Iterator for TScaleIntoIter<T, C>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Default + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.gen_len > 0 {
            self.gen_len -= 1;
            let last_value = self
                .array
                .iter()
                .zip(self.weight.iter().rev())
                .fold(T::default(), |acc, (a, b)| acc + a.clone() * b.clone());
            let first_value = self.array[0].clone();
            (0..C - 1).for_each(|index| self.array[index] = self.array[index + 1].clone());
            self.array[C - 1] = last_value;
            Some(first_value)
        } else {
            None
        }
    }
}

impl<T, const C: usize> IntoIterator for TScale<T, C>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Default + Debug,
{
    type Item = T;
    type IntoIter = TScaleIntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        let Self {
            array,
            weight,
        } = self;
        TScaleIntoIter {
            array: ManuallyDrop::new(array),
            weight: ManuallyDrop::new(weight),
            gen_len:DEFAULT_GEN_LEN,
        }
    }
}


/// compute rate with an+1 and an
pub fn compute_rate_with_data<T, const C: usize>(
    count: usize,
    array: [T; C],
    weight: [T; C],
) -> impl Iterator<Item = T>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Default + Div<Output = T> + Debug,
    TScale<T, C>: Default,
{
    let take_list = TScale::<T, C>::new_with_config(array, weight)
        .into_iter()
        .take(count);
    take_list.scan(None, |v, next| {
        v.replace(next.clone())
            .map_or_else(|| Some(T::default()), |v| Some(next / v))
    })
}


#[cfg(test)]
mod tests {
    use approximately::ApproxEq;

    use super::*;

    #[test]
    fn test_sequence() {
        
        let array = [0., 1.0];
        let weight = [1., 1.];
        // Fibonacci sequence
        compute_rate_with_data(50,array,weight).last().unwrap().assert_approx(1.618034);
    }
}
