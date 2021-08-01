// See https://internals.rust-lang.org/t/pre-rfc-tryfromiterator-and-try-collect-to-enable-collecting-to-arrays/14423/14
use core::mem::{self, MaybeUninit};

pub trait TryFromIterator<A>: Sized {
    type Error;

    fn try_from_iter<T: IntoIterator<Item = A>>(iter: T) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotEnoughItems {
    missing: usize,
}

impl<A, const N: usize> TryFromIterator<A> for [A; N] {
    type Error = NotEnoughItems;

    fn try_from_iter<T: IntoIterator<Item = A>>(iter: T) -> Result<Self, Self::Error> {
        let mut array: [MaybeUninit<A>; N] = MaybeUninit::uninit_array();
        let mut iterator = iter.into_iter();

        for (i, item) in array.iter_mut().enumerate() {
            if let Some(value) = iterator.next() {
                *item = MaybeUninit::new(value);
            } else {
                return Err(NotEnoughItems { missing: N - i });
            }
        }

        // One can not simply use mem::transmute, because of this issue:
        // https://github.com/rust-lang/rust/issues/61956
        let result: [A; N] = unsafe {
            // assert that we have exclusive ownership of the array
            let pointer: *mut [A; N] = &mut array as *mut _ as *mut [A; N];
            let result: [A; N] = pointer.read();
            // forget about the old array
            mem::forget(array);

            result
        };

        Ok(result)
    }
}

impl<A, const N: usize> TryFromIterator<A> for [Option<A>; N] {
    type Error = !;

    fn try_from_iter<T: IntoIterator<Item = A>>(iter: T) -> Result<Self, Self::Error> {
        let mut array: [MaybeUninit<Option<A>>; N] = MaybeUninit::uninit_array();
        let mut iterator = iter.into_iter();

        for item in array.iter_mut() {
            *item = MaybeUninit::new(iterator.next());
        }

        let result: [Option<A>; N] = unsafe {
            // assert that we have exclusive ownership of the array
            let pointer: *mut [Option<A>; N] = &mut array as *mut _ as *mut [Option<A>; N];
            let result: [Option<A>; N] = pointer.read();

            // forget about the old array
            mem::forget(array);

            result
        };

        Ok(result)
    }
}

pub trait IteratorExt: Iterator {
    fn try_collect<B: TryFromIterator<Self::Item>>(self) -> Result<B, B::Error>
    where
        Self: Sized,
    {
        TryFromIterator::try_from_iter(self)
    }
}

impl<I: Iterator> IteratorExt for I {}
