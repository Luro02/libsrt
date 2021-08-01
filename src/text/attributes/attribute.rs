use core::convert::TryFrom;

use super::ParseAttributeError;
use crate::utils::Spanned;
use crate::{Buffer, Serialize};

// https://www.w3.org/community/webed/wiki/HTML/Training/Tag_syntax
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attribute<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> Attribute<'a> {
    // TODO: do not ignore doc examples
    // TODO: must_use attribute
    /// Creates a new [`Attribute`] from the provided `name` and its associated
    /// `value`.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// use libsrt::text::Attribute;
    ///
    /// let attribute = Attribute::new("color", Some("rgb(12, 12, 22)"));
    /// ```
    pub fn new(name: &'a str, value: Option<&'a str>) -> Self { Self { name, value } }

    /// Returns the `name` of the `Attribute`.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// use libsrt::text::Attribute;
    ///
    /// let attribute = Attribute::new("color", Some("#AABBCC"));
    ///
    /// assert_eq!(attribute.name(), "color");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str { &self.name }

    /// Returns the `value` of the `Attribute`.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// use libsrt::text::Attribute;
    ///
    /// let attribute = Attribute::new("color", Some("#AABBCC"));
    ///
    /// assert_eq!(attribute.value(), Some("#AABBCC"));
    /// ```
    #[must_use]
    pub fn value(&self) -> Option<&str> { self.value.as_deref() }

    // This function is for `Attributes::try_from(&str)`
    pub(crate) fn parse(
        string: Spanned<&'a str>,
    ) -> Result<(Spanned<&'a str>, Self), ParseAttributeError> {
        let (name, value) = string.split_once('=');

        if let Some(value) = value {
            Ok((
                name,
                Attribute {
                    name: name.into_inner().into(),
                    value: Some(remove_quotes(value)?.into_inner().into()),
                },
            ))
        } else {
            // The value, along with the "=" character, can be omitted altogether if the
            // value is an empty string.
            Ok((
                name,
                Attribute {
                    name: name.into_inner().into(),
                    value: None,
                },
            ))
        }
    }
}

/// Removes either single or double quotes from the start/end of the provided
/// `string`. If the provided `string` is not quoted, the input will be
/// returned.
///
/// ## Errors
///
/// Returns `ParseAttributeError::InvalidQuote` if the provided string starts
/// with a quote, but does not end with the same one.
fn remove_quotes(string: Spanned<&str>) -> Result<Spanned<&str>, ParseAttributeError> {
    if let Some(string) = string.remove_start_end('"', '"') {
        return Ok(string);
    } else if let Some(string) = string.remove_start_end('\'', '\'') {
        return Ok(string);
    } else if string.starts_with('"')
        || string.ends_with('"')
        || string.starts_with('\'')
        || string.ends_with('\'')
    {
        return Err(ParseAttributeError::invalid_quote(string.range()));
    }

    Ok(string)
}

impl<'a> TryFrom<Spanned<&'a str>> for Attribute<'a> {
    type Error = ParseAttributeError;

    // TODO: use Spanned
    fn try_from(value: Spanned<&'a str>) -> Result<Self, Self::Error> {
        Self::parse(value).map(|(_, a)| a)
    }
}

impl<'a> TryFrom<&'a str> for Attribute<'a> {
    type Error = ParseAttributeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        //
        Self::parse(Spanned::new(value)).map(|(_, a)| a)
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for Attribute<'a> {
    fn from(value: (&'a str, Option<&'a str>)) -> Self { Self::new(value.0, value.1) }
}

impl<'a, B: Buffer> Serialize<B> for Attribute<'a> {
    type Error = B::Error;

    fn serialize(&self, mut buffer: B) -> Result<(), Self::Error> {
        buffer.write_str(&self.name)?;

        if let Some(value) = &self.value {
            let mut needs_quotes = None;

            // > In HTML, the attribute value can remain unquoted
            // > if it doesn't contain spaces or any of the following
            // > characters: " ' ` = < or >. Otherwise, it has to be
            // > quoted using either single or double quotes.
            //
            // https://www.w3.org/community/webed/wiki/HTML/Training/Tag_syntax
            if value.contains(' ')
                || value.contains('\'')
                || value.contains('`')
                || value.contains('=')
                || value.contains('<')
                || value.contains('>')
            {
                if value.contains('"') {
                    needs_quotes = Some('\'');
                } else {
                    needs_quotes = Some('"');
                }
            } else if value.contains('"') {
                // if the value contains a double quote,
                // it has to be escaped with single quotes
                needs_quotes = Some('\'');
            }

            if let Some(c) = needs_quotes {
                buffer.reserve('='.len_utf8() + c.len_utf8() * 2 + value.len())?;
                write!(buffer, "={}{}{}", c, value, c)?;
            } else {
                buffer.reserve('='.len_utf8() + value.len())?;
                write!(buffer, "={}", value)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use crate::serialize::SerializeToString;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            Attribute::try_from("key=value"),
            Ok(Attribute::new("key", Some("value")))
        );

        assert_eq!(
            Attribute::try_from("key=\"value\""),
            Ok(Attribute::new("key", Some("value")))
        );

        assert_eq!(
            Attribute::try_from("key='value'"),
            Ok(Attribute::new("key", Some("value")))
        );

        assert_eq!(
            Attribute::try_from("key=\" ' ` = < or >\""),
            Ok(Attribute::new("key", Some(" ' ` = < or >")))
        );

        assert_eq!(
            Attribute::try_from("key=' \" ` = < or >'"),
            Ok(Attribute::new("key", Some(" \" ` = < or >")))
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_serialize() {
        assert_eq!(
            Attribute::new("key", Some("value")).serialize_to_string(),
            Ok("key=value".to_string())
        );

        assert_eq!(
            Attribute::new("key", Some(" ' ` = < or >")).serialize_to_string(),
            Ok("key=\" ' ` = < or >\"".to_string())
        );

        assert_eq!(
            Attribute::new("key", Some(" \" ` = < or >")).serialize_to_string(),
            Ok("key=' \" ` = < or >'".to_string())
        );
    }
}
