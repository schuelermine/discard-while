#![no_std]
//! Iterator utility for getting the first non-matching element,
//! discarding any others, and the amount of discarded elements.
//!
//! To use, either `use discard_while::discard_while` to get the function,
//! or `use discard_while::DiscardWhile` to get the convenience trait.

/// Advance an iterator as long as a condition on the yielded items holds.
/// Returns the first item that no longer satisfies the condition, if any,
/// and the number of items discarded.
///
/// This is similar to a combination of [`find`] and [`position`].
///
/// # Overflow Behavior
///
/// The method does no guarding against overflows, so if there are more than `usize::MAX`
/// non-matching elements, it either produces the wrong result or panics.
/// If debug assertions are enabled, a panic is guaranteed.
///
/// # Panics
///
/// This function might panic if the iterator has more than `usize::MAX` non-matching elements.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// # use discard_while::discard_while;
/// let mut range = 1..=10;
/// let result = discard_while(&mut range, |&n| n != 5);
/// assert_eq!(result, (Some(5), 4));
/// assert_eq!(range, 6..=10);
/// ```
///
/// If the iterator ends before an item that does not fulfill the condition
/// is encountered, [`None`] is returned as the first return value.
///
/// ```
/// # use discard_while::discard_while;
/// let mut range = 1..=10;
/// let result = discard_while(&mut range, |&n| true);
/// assert_eq!(result, (None, 10));
/// assert!(range.is_empty());
/// ```
///
/// If the first element that is encountered does not fulfill the condition,
/// `0` is returned as the second return value.
///
/// ```
/// # use discard_while::discard_while;
/// let mut range = 1..=10;
/// let result = discard_while(&mut range, |&n| false);
/// assert_eq!(result, (Some(1), 0));
/// assert_eq!(range, 2..=10);
/// ```
///
/// [`find`]: Iterator::find
/// [`position`]: Iterator::position
pub fn discard_while<T>(
    iter: &mut impl Iterator<Item = T>,
    mut cond: impl FnMut(&T) -> bool,
) -> (Option<T>, usize) {
    let mut i = 0;
    while let Some(next) = iter.next() {
        if !cond(&next) {
            return (Some(next), i);
        }
        i += 1;
    }
    (None, i)
}

/// Convenience trait to allow using [`discard_while`] as a method.
/// This trait is implemented for every [`Iterator`].
pub trait DiscardWhile: Iterator {
    /// Advance the iterator as long as a condition on the yielded items holds.
    /// Returns the first item that no longer satisfies the condition, if any,
    /// and the number of items discarded.
    ///
    /// This is similar to a combination of [`find`] and [`position`].
    ///
    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so if there are more than `usize::MAX`
    /// non-matching elements, it either produces the wrong result or panics.
    /// If debug assertions are enabled, a panic is guaranteed.
    ///
    /// # Panics
    ///
    /// This function might panic if the iterator has more than `usize::MAX` non-matching elements.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use discard_while::DiscardWhile;
    /// let mut range = 1..=10;
    /// let result = range.discard_while(|&n| n != 5);
    /// assert_eq!(result, (Some(5), 4));
    /// assert_eq!(range, 6..=10);
    /// ```
    ///
    /// If the iterator ends before an item that does not fulfill the condition
    /// is encountered, [`None`] is returned as the first return value.
    ///
    /// ```
    /// # use discard_while::DiscardWhile;
    /// let mut range = 1..=10;
    /// let result = range.discard_while(|&n| true);
    /// assert_eq!(result, (None, 10));
    /// assert!(range.is_empty());
    /// ```
    /// If the first element that is encountered does not fulfill the condition,
    /// `0` is returned as the second return value.
    ///
    /// ```
    /// # use discard_while::DiscardWhile;
    /// let mut range = 1..=10;
    /// let result = range.discard_while(|&n| false);
    /// assert_eq!(result, (Some(1), 0));
    /// assert_eq!(range, 2..=10);
    /// ```
    /// [`find`]: Iterator::find
    /// [`position`]: Iterator::position
    fn discard_while(
        &mut self,
        cond: impl FnMut(&Self::Item) -> bool,
    ) -> (Option<Self::Item>, usize)
    where
        Self: Sized,
    {
        discard_while(self, cond)
    }
}

impl<T: Iterator> DiscardWhile for T {}
