use core::str::pattern::Pattern;

use super::pattern::{IgnoringPattern, PatternExt};
use super::split::SplitNInternal;
use crate::utils::{Span, Spanned};

#[must_use]
pub struct SplitIterN<'a, P: Pattern<'a>> {
    iterator: SplitNInternal<'a, IgnoringPattern<P>>,
    span: Option<Span>,
}

impl<'a, P: Pattern<'a>> SplitIterN<'a, P> {
    pub fn new_spanned(n: usize, string: Spanned<&'a str>, pattern: P) -> Self {
        Self {
            iterator: SplitNInternal {
                iter: super::split::SplitInternal {
                    start: 0,
                    end: string.len(),
                    matcher: pattern.ignoring().into_searcher(&string),
                    allow_trailing_empty: true,
                    finished: false,
                },
                count: n,
            },
            span: string.span(),
        }
    }
}

impl<'a, P: Pattern<'a>> Iterator for SplitIterN<'a, P> {
    type Item = Spanned<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|spanned| {
            spanned.map_span(|optspan| optspan.map(|span| span + self.span.map_or(0, Span::start)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_splitn_char_iterator() {
        let data = Spanned::from(("\nMäry häd ä little lämb\nLittle lämb\n", 0..));

        let mut iterator = SplitIterN::new_spanned(4, data, ' ');
        assert_eq!(iterator.next(), Some(("\nMäry", 0..).into()));
        assert_eq!(iterator.next(), Some(("häd", 7..).into()));
        assert_eq!(iterator.next(), Some(("ä", 12..).into()));
        assert_eq!(
            iterator.next(),
            Some(("little lämb\nLittle lämb\n", 15..).into())
        );
        assert_eq!(iterator.next(), None);

        let mut iterator = SplitIterN::new_spanned(4, data, 'ä');
        assert_eq!(iterator.next(), Some(("\nM", 0..).into()));
        assert_eq!(iterator.next(), Some(("ry h", 4..).into()));
        assert_eq!(iterator.next(), Some(("d ", 10..).into()));
        assert_eq!(
            iterator.next(),
            Some((" little lämb\nLittle lämb\n", 14..).into())
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_splitn_quotes() {
        let mut iterator = SplitIterN::new_spanned(2, ("hello,world,yeah", 0..).into(), ',');

        assert_eq!(iterator.next(), Some(("hello", 0..).into()));
        assert_eq!(iterator.next(), Some(("world,yeah", 6..).into()));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_splitn_quotes_2() {
        let mut iterator = SplitIterN::new_spanned(2, ("", 0..).into(), ',');

        assert_eq!(iterator.next(), Some(("", 0..).into()));
        assert_eq!(iterator.next(), None);
    }
}
