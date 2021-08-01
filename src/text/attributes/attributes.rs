use core::convert::TryFrom;

use super::{Attribute, ParseAttributeError};
use crate::utils::{Spanned, SplitIter};

// what it could look like:
// Attributes<Spanned<&'a str>>
// Attributes<&'a str>
// Attributes<Attribute<'a>>
// Attributes<[Attribute<'a>; N]>
// Attributes<&'a [Attribute<'a>]>
// Attributes<Vec<Attribute<'a>>>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Attributes<T>(T);

impl<'a> Attributes<Spanned<&'a str>> {
    pub fn new_spanned(string: &'a str, start: usize) -> Self {
        Self(Spanned::new(string).with_span(start..start + string.len()))
    }

    pub fn attributes<'b>(&'b self) -> LazyAttributesIter<'b> {
        //
        LazyAttributesIter::new(self.0)
    }
}

impl<T> From<T> for Attributes<T> {
    fn from(value: T) -> Self { Self(value) }
}

#[derive(Debug)]
pub struct LazyAttributesIter<'a>(SplitIter<'a, char>);

impl<'a> LazyAttributesIter<'a> {
    pub(crate) fn new(value: Spanned<&'a str>) -> Self {
        // TODO: shouldnt you split at a ,?
        Self(value.split('='))
    }
}

impl<'a> Iterator for LazyAttributesIter<'a> {
    type Item = Result<Attribute<'a>, ParseAttributeError>;

    fn next(&mut self) -> Option<Self::Item> {
        let string = self.0.next()?;

        Some(Attribute::try_from(string))
    }
}
