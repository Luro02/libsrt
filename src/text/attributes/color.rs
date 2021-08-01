use core::convert::TryFrom;

use super::ColorError;
use crate::utils::Spanned;
use crate::{Buffer, Serialize, SerializeWithConfig};

#[derive(Debug, Clone, PartialEq)]
pub enum Color<'a> {
    Rgb { red: u8, green: u8, blue: u8 },
    Name(&'a str),
}

impl<'a> TryFrom<Spanned<&'a str>> for Color<'a> {
    type Error = ColorError;

    fn try_from(value: Spanned<&'a str>) -> Result<Self, Self::Error> {
        if value.starts_with('#') {
            if let [Some(red), Some(green), Some(blue)] = value.sub_ranges([1..3, 3..5, 5..7]) {
                let red = red.parse_radix_u8(16)?;
                let green = green.parse_radix_u8(16)?;
                let blue = blue.parse_radix_u8(16)?;

                Ok(Self::Rgb { red, green, blue })
            } else {
                Err(ColorError::invalid_format(value.range()))
            }
        } else if value.starts_with("rgb") {
            // parses a string of the following format: "rgb({red}, {green}, {blue})"
            // (the {color} is a decimal number of type u8)

            // remove "rgb" and the round braces
            if let Some(value) = &value.get(3..).and_then(|s| s.remove_start_end('(', ')')) {
                if let [Some(red), Some(green), Some(blue)] = value.split_at_most::<_, 3>(',') {
                    let red = red.trim().parse_radix_u8(10)?;
                    let green = green.trim().parse_radix_u8(10)?;
                    let blue = blue.trim().parse_radix_u8(10)?;

                    Ok(Self::Rgb { red, green, blue })
                } else {
                    Err(ColorError::invalid_format(value.range()))
                }
            } else {
                Err(ColorError::invalid_rgb_string(value.range()))
            }
        } else {
            Ok(Self::Name(&value))
        }
    }
}

// TODO: remove?
impl<'a> TryFrom<&'a str> for Color<'a> {
    type Error = ColorError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> { Self::try_from(Spanned::new(value)) }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SerializeColorConfig {
    UpperHex,
    LowerHex,
    Rgb,
}

impl Default for SerializeColorConfig {
    #[inline]
    fn default() -> Self { Self::LowerHex }
}

impl<'a, B: Buffer> Serialize<B> for Color<'a> {
    type Error = B::Error;

    fn serialize(&self, buffer: B) -> Result<(), Self::Error> {
        Self::serialize_with_config(self, buffer, &SerializeColorConfig::default())
    }
}

impl<'a, B: Buffer> SerializeWithConfig<B> for Color<'a> {
    type Config = SerializeColorConfig;
    type Error = B::Error;

    fn serialize_with_config(
        &self,
        mut buffer: B,
        config: &Self::Config,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Rgb { red, green, blue } => {
                match config {
                    Self::Config::UpperHex => {
                        buffer.reserve(3 * 2)?;
                        write!(buffer, "#{:02X?}{:02X?}{:02X?}", red, green, blue)?;
                    }
                    Self::Config::LowerHex => {
                        buffer.reserve(3 * 2)?;
                        write!(buffer, "#{:02x?}{:02x?}{:02x?}", red, green, blue)?;
                    }
                    Self::Config::Rgb => {
                        buffer.reserve(4 + 3 + 2 + 3 + 2 + 3 + 1)?;
                        write!(buffer, "rgb({}, {}, {})", red, green, blue)?;
                    }
                }
            }
            Self::Name(name) => {
                buffer.write_str(name)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use crate::serialize::{SerializeToString, SerializeWithConfigToString};
    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_serialize_color_config() {
        assert_eq!(
            SerializeColorConfig::default(),
            SerializeColorConfig::LowerHex
        );
    }

    #[test]
    fn test_color_from_str_error() {
        assert_eq!(
            Color::try_from("#GGFFFF"),
            Err(ColorError::from(
                Spanned::new(u8::from_str_radix("GG", 16).unwrap_err()).with_span(1..3)
            ))
        );

        assert_eq!(
            Color::try_from("#FFGGFF"),
            Err(ColorError::from(
                Spanned::new(u8::from_str_radix("GG", 16).unwrap_err()).with_span(3..5)
            ))
        );

        assert_eq!(
            Color::try_from("#FFFFGG"),
            Err(ColorError::from(
                Spanned::new(u8::from_str_radix("GG", 16).unwrap_err()).with_span(5..7)
            ))
        );
    }

    #[test]
    fn test_color_from_str_hex() {
        assert_eq!(
            Color::try_from("#AABBCC"),
            Ok(Color::Rgb {
                red: 0xAA,
                green: 0xBB,
                blue: 0xCC,
            })
        );

        assert_eq!(
            Color::try_from("#009900"),
            Ok(Color::Rgb {
                red: 0x00,
                green: 0x99,
                blue: 0x00,
            })
        );
    }

    #[test]
    fn test_color_from_str_rgb() {
        assert_eq!(
            Color::try_from("rgb(0,0,0)"),
            Ok(Color::Rgb {
                red: 0,
                green: 0,
                blue: 0,
            })
        );

        assert_eq!(
            Color::try_from("rgb(0,153,0)"),
            Ok(Color::Rgb {
                red: 0,
                green: 153,
                blue: 0,
            })
        );

        assert_eq!(
            Color::try_from("rgb(255, 255, 255)"),
            Ok(Color::Rgb {
                red: 255,
                green: 255,
                blue: 255,
            })
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_serialize() {
        assert_eq!(
            Color::Rgb {
                red: 0xFF,
                green: 0xFF,
                blue: 0xFF,
            }
            .serialize_to_string()
            .unwrap(),
            "#ffffff".to_string()
        );

        assert_eq!(
            Color::Name("red".into()).serialize_to_string().unwrap(),
            "red".to_string()
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_serialize_with_config() {
        assert_eq!(
            Color::Rgb {
                red: 0xAB,
                green: 0xCD,
                blue: 0xEF,
            }
            .serialize_with_config_to_string(&SerializeColorConfig::UpperHex),
            Ok("#ABCDEF".to_string())
        );

        assert_eq!(
            Color::Rgb {
                red: 0xAB,
                green: 0xCD,
                blue: 0xEF,
            }
            .serialize_with_config_to_string(&SerializeColorConfig::LowerHex),
            Ok("#abcdef".to_string())
        );

        assert_eq!(
            Color::Rgb {
                red: 0xAB,
                green: 0xCD,
                blue: 0xEF,
            }
            .serialize_with_config_to_string(&SerializeColorConfig::Rgb),
            Ok(format!("rgb({}, {}, {})", 0xAB, 0xCD, 0xEF))
        );

        assert_eq!(
            Color::Rgb {
                red: 0x0A,
                green: 0x0B,
                blue: 0x0C,
            }
            .serialize_with_config_to_string(&SerializeColorConfig::UpperHex),
            Ok("#0A0B0C".to_string())
        );

        assert_eq!(
            Color::Rgb {
                red: 0x0A,
                green: 0x0B,
                blue: 0x0C,
            }
            .serialize_with_config_to_string(&SerializeColorConfig::LowerHex),
            Ok("#0a0b0c".to_string())
        );
    }
}
