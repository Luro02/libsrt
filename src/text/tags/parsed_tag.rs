use core::convert::TryFrom;

use super::super::Attributes;
use super::ParseTagError;
use crate::utils::Spanned;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TagKind {
    Braced,
    AngularBrackets,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedTag<'name, T> {
    kind: TagKind,
    name: &'name str,
    is_open: bool,
    attributes: Option<Attributes<T>>,
}

impl<'name, T> ParsedTag<'name, T> {
    pub fn new_open(name: &'name str, kind: TagKind, attributes: Option<Attributes<T>>) -> Self {
        Self {
            kind,
            name,
            is_open: true,
            attributes,
        }
    }

    pub fn new_closed(name: &'name str, kind: TagKind, attributes: Option<Attributes<T>>) -> Self {
        Self {
            kind,
            name,
            is_open: false,
            attributes,
        }
    }

    pub fn kind(&self) -> TagKind { self.kind }
}

macro_rules! implement_default_tags {
    ( $(
        $name:ident {
            name_open => $open:ident,
            name_closed => $closed:ident,
            string => $string:expr
            $(,)*
        }
    ),+ $(,)*) => {
        impl<'name, T> ParsedTag<'name, T> {
            $(
                pub fn $open (kind: TagKind) -> Self {
                    Self::new_open($string, kind, None)
                }

                pub fn $closed (kind: TagKind) -> Self {
                    Self::new_closed($string, kind, None)
                }
            )*
        }
    };
}

// TODO: ref tag? (see parser_02 test)

// TODO: test that this works correctly :)
implement_default_tags! {
    bold {
        name_open => bold_open,
        name_closed => bold_closed,
        string => "b",
    },
    italic {
        name_open => italic_open,
        name_closed => italic_closed,
        string => "i",
    },
    underlined {
        name_open => underlined_open,
        name_closed => underlined_closed,
        string => "u",
    },
}

impl<'a> TryFrom<Spanned<&'a str>> for ParsedTag<'a, Spanned<&'a str>> {
    type Error = ParseTagError;

    fn try_from(value: Spanned<&'a str>) -> Result<Self, Self::Error> {
        let (string, kind) = {
            if let Some(string) = value.remove_start_end('<', '>') {
                (string, TagKind::AngularBrackets)
            } else if let Some(string) = value.remove_start_end('{', '}') {
                (string, TagKind::Braced)
            } else {
                // TODO: rename
                return Err(Self::Error::missing_brackets(0..value.len()));
            }
        };

        let is_open = !string.starts_with('/');

        let (mut name, attributes) = string.split_once(' ');

        if !is_open {
            // TODO: trim only one '/'
            name = name.trim_start_matches('/');
        }

        Ok(Self {
            kind,
            name: &name,
            is_open,
            attributes: attributes.map(Attributes::from),
        })
    }
}
