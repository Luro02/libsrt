use core::convert::TryFrom;
use core::fmt;
use core::iter::Peekable;
use core::ops::{Deref, RangeFrom};
use core::str::CharIndices;

use super::tags::{ParseTagError, ParsedTag};
use crate::utils::Spanned;

/// `Text` of a subtitle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text<'a> {
    text: Spanned<&'a str>,
}

impl<'a> Text<'a> {
    /// Returns the underlying string.
    ///
    /// ## Example
    ///
    /// ```
    /// use libsrt::Text;
    ///
    /// let text = Text::from("<b>example text</b>");
    ///
    /// assert_eq!(text.as_raw(), "<b>example text</b>");
    /// ```
    #[inline]
    #[must_use]
    pub fn as_raw(&self) -> &str { &self.text }
}

impl PartialEq<str> for Text<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool { *self.text == other }
}

impl PartialEq<&str> for Text<'_> {
    #[inline]
    fn eq(&self, other: &&str) -> bool { self == *other }
}

impl<'a> Deref for Text<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target { &*self.text }
}

impl<'a> From<(&'a str, RangeFrom<usize>)> for Text<'a> {
    fn from(value: (&'a str, RangeFrom<usize>)) -> Self { Self { text: value.into() } }
}

// TODO: remove?
impl<'a> From<&'a str> for Text<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            text: (value, 0..).into(),
        }
    }
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.text) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextEvent<'a> {
    Tag(ParsedTag<'a, Spanned<&'a str>>),
    Text(&'a str),
}

impl<'a> IntoIterator for Text<'a> {
    type IntoIter = TextIter<'a>;
    type Item = Result<TextEvent<'a>, ParseTagError>;

    fn into_iter(self) -> Self::IntoIter { TextIter::new(self.text) }
}

#[derive(Debug)]
#[must_use]
pub struct TextIter<'a> {
    text: Spanned<&'a str>,
    char_indices: Peekable<CharIndices<'a>>,
}

impl<'a> TextIter<'a> {
    pub(crate) fn new(text: Spanned<&'a str>) -> Self {
        Self {
            text,
            char_indices: text.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for TextIter<'a> {
    type Item = Result<TextEvent<'a>, ParseTagError>;

    fn next(&mut self) -> Option<Self::Item> {
        let iterator = self.char_indices.by_ref();

        let (start_index, start_char) = *iterator.peek()?;

        // TODO: braces should be possible too!

        // check if the next event is a tag
        if start_char == '<' {
            iterator.next();

            let mut outside_double_quotes = true;
            let mut outside_single_quotes = true;

            for (next_index, next_char) in iterator {
                if next_char == '>' && outside_double_quotes && outside_single_quotes {
                    let tag = self.text.get(start_index..=next_index)?;

                    return Some(ParsedTag::try_from(tag).map(TextEvent::Tag));
                } else if next_char == '"' {
                    outside_double_quotes = !outside_double_quotes;
                } else if next_char == '\'' {
                    outside_single_quotes = !outside_single_quotes;
                }
            }

            // could not find a closing '>', therefore the rest of self.text
            // is simple text that starts with a '<'
            return Some(Ok(TextEvent::Text(&self.text[start_index..])));
        } else {
            // this is text until the next char would be '<'
            loop {
                if let Some((next_index, next_char)) = iterator.peek().copied() {
                    if next_char == '<' {
                        return Some(Ok(TextEvent::Text(&self.text[start_index..next_index])));
                    } else {
                        iterator.next();
                    }
                } else {
                    return Some(Ok(TextEvent::Text(&self.text[start_index..])));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Attributes;
    use super::super::TagKind;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_text_iter() {
        let mut iterator = Text::from(concat!(
            "<font color=#AABBCC>text with color #AABBCC</font>\n",
            "<b>fat text</b><i><u>underlined and italic text</i>underlined text</u>"
        ))
        .into_iter();

        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_open(
                "font",
                TagKind::AngularBrackets,
                Some(Attributes::new_spanned("color=#AABBCC", 6))
            ))))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Text("text with color #AABBCC")))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_closed(
                "font",
                TagKind::AngularBrackets,
                None
            ))))
        );
        assert_eq!(iterator.next(), Some(Ok(TextEvent::Text("\n"))));
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::bold_open(
                TagKind::AngularBrackets
            ))))
        );
        assert_eq!(iterator.next(), Some(Ok(TextEvent::Text("fat text"))));
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::bold_closed(
                TagKind::AngularBrackets
            ))))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::italic_open(
                TagKind::AngularBrackets
            ))))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::underlined_open(
                TagKind::AngularBrackets
            ))))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Text("underlined and italic text")))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::italic_closed(
                TagKind::AngularBrackets
            ))))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Text("underlined text")))
        );
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::underlined_closed(
                TagKind::AngularBrackets
            ))))
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_text_iter_ignore() {
        let mut iterator = Text::from(concat!(
            "<font color=\"<\">text></font>\n",
            "<<b>fat> text</<b>"
        ))
        .into_iter();

        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_open(
                "font",
                TagKind::AngularBrackets,
                Some(Attributes::new_spanned("color=\"<\"", 6))
            ))))
        );
        assert_eq!(iterator.next(), Some(Ok(TextEvent::Text("text>"))));
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_closed(
                "font",
                TagKind::AngularBrackets,
                None
            ))))
        );
        assert_eq!(iterator.next(), Some(Ok(TextEvent::Text("\n"))));
        assert_eq!(
            iterator.next(),
            // TODO: this is not a valid tag!
            Some(Ok(TextEvent::Tag(ParsedTag::new_open(
                "<b",
                TagKind::AngularBrackets,
                None
            ))))
        );
        assert_eq!(iterator.next(), Some(Ok(TextEvent::Text("fat> text"))));
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_closed(
                "<b",
                TagKind::AngularBrackets,
                None
            ))))
        );

        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_text_iter_quoted_string() {
        let mut iterator = Text::from("<font color=\">\">text></font>").into_iter();

        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_open(
                "font",
                TagKind::AngularBrackets,
                Some(Attributes::new_spanned("color=\">\"", 6))
            ))))
        );
        assert_eq!(iterator.next(), Some(Ok(TextEvent::Text("text>"))));
        assert_eq!(
            iterator.next(),
            Some(Ok(TextEvent::Tag(ParsedTag::new_closed(
                "font",
                TagKind::AngularBrackets,
                None
            ))))
        );

        assert_eq!(iterator.next(), None);
    }
}
