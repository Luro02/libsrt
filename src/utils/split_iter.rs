use core::fmt;
use core::str::pattern::{Pattern, Searcher};

use crate::utils::Spanned;

use super::pattern::{IgnoringPattern, PatternExt};

#[must_use]
pub struct SplitIter<'a, P: Pattern<'a>> {
    pub(super) iterator: super::split::SplitInternal<'a, IgnoringPattern<P>>,
    // where the string is located in the original string (this is the start of a span)
    pub(super) start: usize,
}

impl<'a, P: Pattern<'a>> SplitIter<'a, P> {
    #[must_use]
    fn new_(string: &'a str, pattern: P, start: usize) -> Self {
        Self {
            iterator: super::split::SplitInternal {
                start: 0,
                end: string.len(),
                matcher: pattern.ignoring().into_searcher(string),
                allow_trailing_empty: true,
                finished: false,
            },
            start,
        }
    }

    #[must_use]
    pub fn new(string: &'a str, pattern: P) -> Self { Self::new_(string, pattern, 0) }

    #[must_use]
    pub fn new_spanned(string: Spanned<&'a str>, pattern: P) -> Self {
        let (string, span) = string.into_parts();

        Self::new_(string, pattern, span.start())
    }
}

impl<'a, P: Pattern<'a>> SplitIter<'a, P> {
    #[must_use]
    pub fn split_remaining(&mut self) -> Option<Spanned<&'a str>> {
        Some((self.iterator.as_str(), self.start + self.iterator.start..).into())
    }

    // TODO: as spanned str?
    pub fn as_str(&self) -> &'a str { self.iterator.matcher.haystack() }
}

impl<'a, P: Pattern<'a>> Iterator for SplitIter<'a, P> {
    type Item = Spanned<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let string = self.iterator.next()?;

        // TODO: simplify
        Some(string.map_span(|span| span.map(|s| s + self.start)))
    }
}

impl<'a, P: Pattern<'a>> fmt::Debug for SplitIter<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitIter")
            .field("start", &self.start)
            .field("string", &self.as_str())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_split_char_iterator_no_trailing() {
        let mut iterator = SplitIter::new("\nMäry häd ä little lämb\nLittle lämb\n", '\n');
        assert_eq!(iterator.next(), Some(("", 0..).into()));
        assert_eq!(
            iterator.next(),
            Some(("Märy häd ä little lämb", 1..).into())
        );
        assert_eq!(iterator.next(), Some(("Little lämb", 28..).into()));
        // the span is 40 if "" is before the split char and 41 if it is after
        assert_eq!(iterator.next(), Some(("", 41..).into()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_split_iterator_01() {
        let mut iterator = SplitIter::new(",example,\",\",',',", ',');

        assert_eq!(iterator.next(), Some(("", 0..).into()));
        assert_eq!(iterator.next(), Some(("example", 1..).into()));
        assert_eq!(iterator.next(), Some(("\",\"", 9..).into()));
        assert_eq!(iterator.next(), Some(("','", 13..).into()));
        assert_eq!(iterator.next(), Some(("", 17..).into()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_split_iterator_02() {
        // If a string contains multiple contiguous
        // separators, you will end up with empty strings
        // in the output:
        let mut iterator = SplitIter::new("||||a||b|c", '|');

        assert_eq!(iterator.next(), Some(("", 0..).into()));
        assert_eq!(iterator.next(), Some(("", 1..).into()));
        assert_eq!(iterator.next(), Some(("", 2..).into()));
        assert_eq!(iterator.next(), Some(("", 3..).into()));
        assert_eq!(iterator.next(), Some(("a", 4..).into()));
        assert_eq!(iterator.next(), Some(("", 6..).into()));
        assert_eq!(iterator.next(), Some(("b", 7..).into()));
        assert_eq!(iterator.next(), Some(("c", 9..).into()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_split_iterator_03() {
        // Contiguous separators are separated by the empty string.
        let mut iterator = SplitIter::new("(///)", '/');

        assert_eq!(iterator.next(), Some(("(", 0..).into()));
        assert_eq!(iterator.next(), Some(("", 2..).into()));
        assert_eq!(iterator.next(), Some(("", 3..).into()));
        assert_eq!(iterator.next(), Some((")", 4..).into()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_split_iterator_04() {
        // Separators at the start or end of a string
        // are neighbored by empty strings.
        let data = "010";
        let mut iterator = SplitIter::new(data, '0');

        assert_eq!(iterator.next(), Some(("", 0..).into()));
        assert_eq!(&data[0..0], "");
        assert_eq!(iterator.next(), Some(("1", 1..).into()));
        assert_eq!(&data[1..2], "1");
        assert_eq!(iterator.next(), Some(("", 3..).into()));
        assert_eq!(&data[3..3], "");
        assert_eq!(iterator.next(), None);
    }
}
