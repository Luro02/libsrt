use core::fmt;

use super::buffer::Buffer;

/// Trait alias that contains the bare minimum of constraints that an error
/// should satisfy
pub trait BasicError = fmt::Display + Sized + Send + Sync + 'static;

pub trait Serialize<B: Buffer> {
    type Error: BasicError + From<B::Error>;

    fn serialize(&self, buffer: B) -> Result<(), Self::Error>;
}

// TODO: implement? and use with specialization
pub trait ExactSerializedLength {
    fn len(&self) -> usize;
}

pub trait SerializeWithConfig<B: Buffer> {
    // TODO: why default config?
    type Config: Default + ?Sized;
    type Error: BasicError + From<B::Error>;

    fn serialize_with_config(&self, buffer: B, config: &Self::Config) -> Result<(), Self::Error>;
}

#[cfg(feature = "alloc")]
mod serialize_to_string {
    use super::*;
    use alloc::string::String;

    pub trait SerializeToString: Serialize<String> {
        fn serialize_to_string(&self) -> Result<String, Self::Error>;
    }

    impl<S: Serialize<String>> SerializeToString for S {
        fn serialize_to_string(&self) -> Result<String, Self::Error> {
            let mut buffer = String::new();

            self.serialize(&mut buffer)?;

            Ok(buffer)
        }
    }

    pub trait SerializeWithConfigToString: SerializeWithConfig<String> {
        fn serialize_with_config_to_string(
            &self,
            config: &Self::Config,
        ) -> Result<String, Self::Error>;
    }

    impl<S: SerializeWithConfig<String>> SerializeWithConfigToString for S {
        fn serialize_with_config_to_string(
            &self,
            config: &Self::Config,
        ) -> Result<String, Self::Error> {
            let mut buffer = String::new();

            self.serialize_with_config(&mut buffer, config)?;

            Ok(buffer)
        }
    }
}

#[cfg(feature = "alloc")]
use serialize_to_string::*;
