//! This file is a clone/copy of the rust source code for the Split Iterator
use core::fmt;
use core::str::pattern::{Pattern, Searcher};

use crate::utils::Spanned;

// TODO: simplify

pub(super) struct SplitNInternal<'a, P: Pattern<'a>> {
    pub(super) iter: SplitInternal<'a, P>,
    /// The number of splits remaining
    pub(super) count: usize,
}

impl<'a, P> fmt::Debug for SplitNInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitNInternal")
            .field("iter", &self.iter)
            .field("count", &self.count)
            .finish()
    }
}

impl<'a, P: Pattern<'a>> SplitNInternal<'a, P> {
    #[inline]
    pub fn next(&mut self) -> Option<Spanned<&'a str>> {
        match self.count {
            0 => None,
            1 => {
                self.count = 0;
                self.iter.get_end()
            }
            _ => {
                self.count -= 1;
                self.iter.next()
            }
        }
    }
}

pub struct SplitInternal<'a, P: Pattern<'a>> {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) matcher: P::Searcher,
    pub(super) allow_trailing_empty: bool,
    pub(super) finished: bool,
}

macro_rules! derive_pattern_clone {
    (clone $t:ident with | $s:ident | $e:expr) => {
        impl<'a, P> Clone for $t<'a, P>
        where
            P: Pattern<'a, Searcher: Clone>,
        {
            fn clone(&self) -> Self {
                let $s = self;
                $e
            }
        }
    };
}

derive_pattern_clone! {
    clone SplitInternal
    with |s| SplitInternal { matcher: s.matcher.clone(), ..*s }
}

impl<'a, P> fmt::Debug for SplitInternal<'a, P>
where
    P: Pattern<'a, Searcher: fmt::Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitInternal")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("matcher", &self.matcher)
            .field("allow_trailing_empty", &self.allow_trailing_empty)
            .field("finished", &self.finished)
            .finish()
    }
}

impl<'a, P: Pattern<'a>> SplitInternal<'a, P> {
    #[inline]
    pub fn get_end(&mut self) -> Option<Spanned<&'a str>> {
        if !self.finished && (self.allow_trailing_empty || self.end - self.start > 0) {
            self.finished = true;
            // SAFETY: `self.start` and `self.end` always lie on unicode boundaries.
            unsafe {
                let string = self.matcher.haystack().get_unchecked(self.start..self.end);
                Some(Spanned::new(string).with_span(self.start..self.end))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn next(&mut self) -> Option<Spanned<&'a str>> {
        if self.finished {
            return None;
        }

        let haystack = self.matcher.haystack();
        match self.matcher.next_match() {
            // SAFETY: `Searcher` guarantees that `a` and `b` lie on unicode boundaries.
            Some((a, b)) => unsafe {
                let result = (haystack.get_unchecked(self.start..a), self.start..).into();
                self.start = b;
                Some(result)
            },
            None => self.get_end(),
        }
    }

    #[must_use]
    pub fn haystack(&self) -> &'a str { self.matcher.haystack() }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        // `Self::get_end` doesn't change `self.start`
        if self.finished {
            return "";
        }

        // SAFETY: `self.start` and `self.end` always lie on unicode boundaries.
        unsafe { self.matcher.haystack().get_unchecked(self.start..self.end) }
    }
}
