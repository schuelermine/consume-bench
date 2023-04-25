#![feature(step_trait)]

use core::iter::Step;

fn consume(iter: impl IntoIterator) {
    for _ in iter {}
}

pub fn step_count_from_checked_consume<T: Step>(iter: impl IntoIterator, start: T) -> Option<T> {
    let mut iter = iter.into_iter();
    let result = iter.try_fold(start, |count, _| T::forward_checked(count, 1));
    consume(iter);
    result
}

pub fn step_count_from_checked_last<T: Step>(iter: impl IntoIterator, start: T) -> Option<T> {
    let mut iter = iter.into_iter();
    let result = iter.try_fold(start, |count, _| T::forward_checked(count, 1));
    iter.last();
    result
}
