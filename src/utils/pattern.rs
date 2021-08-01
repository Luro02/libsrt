use core::cmp::PartialOrd;
use core::str::pattern::{Pattern, SearchStep, Searcher};

use logical_pattern::OrPattern;

pub trait PatternExt<'a>: Pattern<'a> {
    fn ignoring(self) -> IgnoringPattern<Self> { IgnoringPattern(self) }

    fn or<'b, P: Pattern<'b>>(self, other: P) -> OrPattern<Self, P> { OrPattern::new(self, other) }
}

impl<'a, P: Pattern<'a>> PatternExt<'a> for P {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IgnoringPattern<T>(pub(super) T);

impl<'a, T: Pattern<'a>> Pattern<'a> for IgnoringPattern<T> {
    type Searcher = IgnoringSearcher<T::Searcher>;

    fn into_searcher(self, haystack: &'a str) -> Self::Searcher {
        IgnoringSearcher::from(self.0.into_searcher(haystack))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct IgnoringSearcher<S> {
    searcher: S,
    reviewer: Reviewer<char, 2>,
}

impl<'a, 'b, S: Searcher<'a>> From<S> for IgnoringSearcher<S> {
    fn from(value: S) -> Self {
        Self {
            searcher: value,
            reviewer: Reviewer::new(['\'', '\"']),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Reviewer<T, const N: usize>([(T, bool); N]);

impl<T, const N: usize> Reviewer<T, N> {
    #[must_use]
    pub fn new(ignore: [T; N]) -> Self { Self(ignore.map(|value| (value, false))) }

    pub fn reset(&mut self) {
        for (_, state) in self.0.iter_mut() {
            *state = false;
        }
    }
}

impl<T: PartialEq<T>, const N: usize> Reviewer<T, N> {
    pub fn visit(&mut self, find: &T) -> bool {
        if let Some((_, state)) = self.0.iter_mut().find(|(value, _)| value == find) {
            *state = !*state;
            *state
        } else {
            false
        }
    }

    // if it is an open token ( token without a closing one ) this returns true
    pub fn partial(&self) -> bool {
        //
        self.0.iter().any(|(_, s)| *s)
    }
}

unsafe impl<'a, S: Searcher<'a>> Searcher<'a> for IgnoringSearcher<S> {
    fn haystack(&self) -> &'a str { self.searcher.haystack() }

    fn next(&mut self) -> SearchStep {
        let step = self.searcher.next();

        if let SearchStep::Match(start, end) = step {
            // TODO: start from the last match?
            for c in self.haystack()[..start].chars() {
                self.reviewer.visit(&c);
            }

            let result = {
                if self.reviewer.partial() {
                    SearchStep::Reject(start, end)
                } else {
                    SearchStep::Match(start, end)
                }
            };

            // TODO: do you have to reset?
            self.reviewer.reset();

            result
        } else {
            step
        }
    }
}

// TODO: splitting "\"hello '\" world" should return ["\"hello '\"", "world"]

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pattern_normal_string() {
        let mut iterator = "this is an example string".split(" ".ignoring());

        assert_eq!(iterator.next(), Some("this"));
        assert_eq!(iterator.next(), Some("is"));
        assert_eq!(iterator.next(), Some("an"));
        assert_eq!(iterator.next(), Some("example"));
        assert_eq!(iterator.next(), Some("string"));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_pattern_single_quotes() {
        let mut iterator = "this is an' 'example string".split(' '.ignoring());

        assert_eq!(iterator.next(), Some("this"));
        assert_eq!(iterator.next(), Some("is"));
        assert_eq!(iterator.next(), Some("an' 'example"));
        assert_eq!(iterator.next(), Some("string"));
        assert_eq!(iterator.next(), None);
    }
}
