// A list of tags supported by srt file format:
// - <b></b>
// - <i></i>
// - <u></u>
// - UnknownTag

mod error;
mod parsed_tag;

pub use error::ParseTagError;
pub use parsed_tag::{ParsedTag, TagKind};
