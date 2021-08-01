use core::convert::TryFrom;
use core::str;
use core::time::Duration;

use crate::parser::ParserError;
use crate::subtitle_iterator::SubtitleIterator;
use crate::text::Text;

// TODO: implement fmt::Display for no_std, see https://github.com/dtolnay/thiserror/pull/64
#[cfg(feature = "std")]
use thiserror::Error;

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum InitError {
    #[cfg_attr(feature = "std", error("subtitle text is empty (\"\")"))]
    MissingSubtitleText,
    #[cfg_attr(feature = "std", error("duration should not be `0s`"))]
    ZeroDuration,
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SubtitleError {
    #[cfg_attr(feature = "std", error("{0}"))]
    Parser(ParserError),
    #[cfg_attr(feature = "std", error("{0}"))]
    Init(InitError),
    #[cfg_attr(feature = "std", error("missing duration"))]
    MissingDuration,
    #[cfg_attr(feature = "std", error("missing text"))]
    MissingText,
    #[cfg_attr(feature = "std", error("missing counter"))]
    MissingCounter,
    #[cfg_attr(feature = "std", error("empty string"))]
    EmptyString,
    #[cfg_attr(
        feature = "std",
        error("encountered multiple subtitle with the same counter")
    )]
    DuplicateEntry,
}

impl From<ParserError> for SubtitleError {
    #[inline]
    fn from(value: ParserError) -> Self { Self::Parser(value) }
}

impl From<InitError> for SubtitleError {
    #[inline]
    fn from(value: InitError) -> Self { Self::Init(value) }
}

/// Initializes a [`Subtitle`].
#[derive(Debug, PartialEq)]
pub struct SubtitleInit<'a> {
    pub counter: usize,
    pub start: Duration,
    pub duration: Duration,
    pub text: Text<'a>,
    // the #[non_exhaustive] attribute does not work with the `..Default::default()` syntax
    #[doc(hidden)]
    pub __non_exhaustive: (),
}

impl<'a> SubtitleInit<'a> {
    /// Initializes the struct and verifies the provided fields.
    ///
    /// # Verification
    ///
    /// The field [`SubtitleInit::duration`] must not be `0s` and
    /// [`SubtitleInit::text`] should not be `""`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsrt::SubtitleInit;
    /// use std::time::Duration;
    ///
    /// let subtitle = SubtitleInit {
    ///     counter: 1,
    ///     start: Duration::from_secs(1),
    ///     duration: Duration::from_secs_f32(1.4),
    ///     text: "Example Subtitle".into(),
    ///     ..SubtitleInit::default()
    /// }
    /// .init()?;
    /// # Ok::<(), libsrt::InitError>(())
    /// ```
    ///
    /// An invalid duration:
    ///
    /// ```
    /// # use libsrt::SubtitleInit;
    /// use libsrt::InitError;
    /// use std::time::Duration;
    ///
    /// assert_eq!(
    ///     SubtitleInit {
    ///         duration: Duration::from_secs(0), // duration must not be zero!
    ///         text: "Example Subtitle".into(),
    ///         ..SubtitleInit::default()
    ///     }
    ///     .init(),
    ///     Err(InitError::ZeroDuration)
    /// );
    /// ```
    ///
    /// An empty text:
    ///
    /// ```
    /// # use libsrt::SubtitleInit;
    /// use libsrt::InitError;
    /// use std::time::Duration;
    ///
    /// assert_eq!(
    ///     SubtitleInit {
    ///         duration: Duration::from_secs(1),
    ///         // a subtitle with no text makes no sense:
    ///         text: "".into(),
    ///         ..SubtitleInit::default()
    ///     }
    ///     .init(),
    ///     Err(InitError::MissingSubtitleText)
    /// );
    /// ```
    #[inline]
    pub fn init(self) -> Result<Subtitle<'a>, InitError> {
        if self.duration == Duration::default() {
            return Err(InitError::ZeroDuration);
        } else if self.text == "" {
            return Err(InitError::MissingSubtitleText);
        }

        Ok(Subtitle {
            counter: self.counter,
            start: self.start,
            duration: self.duration,
            text: self.text,
        })
    }
}

impl Default for SubtitleInit<'_> {
    fn default() -> Self {
        Self {
            counter: 0,
            start: Duration::default(),
            duration: Duration::default(),
            text: "".into(),
            __non_exhaustive: (),
        }
    }
}

/// This struct represents a `Subtitle` for a `SubRipTitle`.
#[derive(Debug, Clone, PartialEq)]
pub struct Subtitle<'a> {
    counter: usize,
    start: Duration,
    duration: Duration,
    text: Text<'a>,
}

impl<'a> Subtitle<'a> {
    /// Each subtitle has a counter associated with it that is returned by this
    /// function.
    ///
    /// This counter can only be modified with the
    /// [`SubRipTitle::insert_subtitle_at`] function.
    ///
    /// [`SubRipTitle::insert_subtitle_at`]:
    /// crate::SubRipTitle::insert_subtitle_at
    #[inline]
    #[must_use]
    pub const fn counter(&self) -> usize { self.counter }

    /// Returns the text associated with this subtitle.
    #[inline]
    #[must_use]
    pub const fn text(&self) -> &Text<'a> { &self.text }

    #[inline]
    #[must_use]
    pub const fn start(&self) -> Duration { self.start }
}

impl<'a> TryFrom<&'a str> for Subtitle<'a> {
    type Error = SubtitleError;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        SubtitleIterator::from(input)
            .next()
            .ok_or(SubtitleError::EmptyString)
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_empty_string() {
        assert_eq!(Subtitle::try_from(""), Err(SubtitleError::EmptyString));
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(
            Subtitle::try_from(concat!(
                "1\n",
                "00:00:01,600 --> 00:00:04,200\n",
                "English (US)\n",
            )),
            Ok(SubtitleInit {
                counter: 1,
                start: Duration::from_secs_f32(1.6),
                duration: Duration::from_secs_f32(4.2 - 1.6),
                text: ("English (US)", 32..).into(),
                ..SubtitleInit::default()
            }
            .init()
            .unwrap())
        );

        assert_eq!(
            Subtitle::try_from(concat!(
                "2\n",
                "00:00:05,900 --> 00:00:07,999\n",
                "This is a subtitle in American English\n",
            )),
            Ok(SubtitleInit {
                counter: 2,
                start: Duration::from_secs(5) + Duration::from_millis(900),
                duration: Duration::from_secs(2) + Duration::from_millis(99),
                text: ("This is a subtitle in American English", 32..).into(),
                ..SubtitleInit::default()
            }
            .init()
            .unwrap())
        );

        assert_eq!(
            Subtitle::try_from(concat!(
                "3\n",
                "00:00:10,000 --> 00:00:14,000\n",
                "Adding subtitles is very easy to do"
            )),
            Ok(SubtitleInit {
                counter: 3,
                start: Duration::from_secs(10),
                duration: Duration::from_secs(14 - 10),
                text: ("Adding subtitles is very easy to do", 32..).into(),
                ..SubtitleInit::default()
            }
            .init()
            .unwrap())
        );
    }
}
