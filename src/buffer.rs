use core::fmt;
use core::marker::{Send, Sized, Sync};

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "lexical-core")]
use core::str;
#[cfg(feature = "lexical-core")]
use lexical_core::Number;

pub trait Buffer: fmt::Write {
    type Error: fmt::Display + Sized + Send + Sync + 'static + From<fmt::Error>;

    /// Reserve at least `n` bytes in the output buffer.
    fn reserve(&mut self, n: usize) -> Result<(), Self::Error>;

    /// Writes an `f32` to the output buffer.
    fn write_f32(&mut self, float: f32) -> Result<(), Self::Error> {
        #[cfg(feature = "lexical-core")]
        {
            let mut buffer = [0u8; f32::FORMATTED_SIZE];
            let string = str::from_utf8(lexical_core::write(float, &mut buffer)).unwrap();
            self.write_str(string)?;
        }
        #[cfg(not(feature = "lexical-core"))]
        {
            write!(self, "{}", float)?;
        }

        Ok(())
    }

    /// Writes an `f64` to the output buffer.
    fn write_f64(&mut self, float: f64) -> Result<(), Self::Error> {
        #[cfg(feature = "lexical-core")]
        {
            let mut buffer = [0u8; f64::FORMATTED_SIZE];
            let string = str::from_utf8(lexical_core::write(float, &mut buffer)).unwrap();

            self.write_str(string)?;
        }
        #[cfg(not(feature = "lexical-core"))]
        {
            write!(self, "{}", float)?;
        }

        Ok(())
    }
}

impl<B: Buffer + ?Sized> Buffer for &mut B {
    type Error = B::Error;

    #[inline]
    fn reserve(&mut self, n: usize) -> Result<(), Self::Error> { (**self).reserve(n) }

    #[inline]
    fn write_f32(&mut self, float: f32) -> Result<(), Self::Error> { (**self).write_f32(float) }

    #[inline]
    fn write_f64(&mut self, float: f64) -> Result<(), Self::Error> { (**self).write_f64(float) }
}

#[cfg(feature = "alloc")]
impl Buffer for String {
    type Error = !;

    fn reserve(&mut self, n: usize) -> Result<(), Self::Error> {
        Self::reserve(self, n);

        Ok(())
    }
}

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    struct MinimalBuffer(pub String);

    impl Buffer for MinimalBuffer {
        type Error = !;

        fn reserve(&mut self, n: usize) -> Result<(), Self::Error> {
            self.0.reserve(n);

            Ok(())
        }

        fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
            self.0.push(c);

            Ok(())
        }
    }

    #[test]
    fn test_default_buffer_write_str() {
        let mut buffer = MinimalBuffer(String::new());

        assert_eq!(buffer.write_str("hello "), Ok(()));
        assert_eq!(buffer.write_str("world"), Ok(()));
        assert_eq!(&buffer.0, "hello world");
    }

    #[test]
    fn test_default_buffer_write_string() {
        let mut buffer = MinimalBuffer(String::new());

        assert_eq!(buffer.write_string(String::from("hello ")), Ok(()));
        assert_eq!(buffer.write_string(String::from("world")), Ok(()));
        assert_eq!(&buffer.0, "hello world");
    }

    #[test]
    fn test_default_buffer_write_f32() {
        let mut buffer = MinimalBuffer(String::new());

        assert_eq!(buffer.write_f32(core::f32::consts::PI), Ok(()));
        assert_eq!(&buffer.0, "3.1415927");
    }

    #[test]
    fn test_default_buffer_write_f64() {
        let mut buffer = MinimalBuffer(String::new());

        assert_eq!(buffer.write_f64(core::f64::consts::PI), Ok(()));
        assert_eq!(&buffer.0, "3.141592653589793");
    }

    #[test]
    fn test_string_buffer_reserve() {
        let mut buffer = String::with_capacity(10);

        assert_eq!(buffer.capacity(), 10);
        assert_eq!(<String as Buffer>::reserve(&mut buffer, 200), Ok(()));
        assert_eq!(buffer.capacity(), 200);
    }

    #[test]
    fn test_string_buffer_write_str() {
        let mut buffer = String::new();

        assert_eq!(<String as Buffer>::write_str(&mut buffer, "hello"), Ok(()));
        assert_eq!(<String as Buffer>::write_str(&mut buffer, " world"), Ok(()));
        assert_eq!(&buffer, "hello world");
    }

    #[test]
    fn test_string_buffer_write_char() {
        let mut buffer = String::new();

        assert_eq!(<String as Buffer>::write_char(&mut buffer, 'h'), Ok(()));
        assert_eq!(<String as Buffer>::write_char(&mut buffer, 'e'), Ok(()));
        assert_eq!(<String as Buffer>::write_char(&mut buffer, 'l'), Ok(()));
        assert_eq!(<String as Buffer>::write_char(&mut buffer, 'l'), Ok(()));
        assert_eq!(<String as Buffer>::write_char(&mut buffer, 'o'), Ok(()));
        assert_eq!(&buffer, "hello");
    }
}
