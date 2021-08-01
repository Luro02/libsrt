use core::time::Duration;

use crate::Text;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event<'a> {
    Counter(usize),
    Duration {
        start: Duration,
        end: Duration,
    },
    Text(Text<'a>),
    /// Returned if an empty line has been encountered, which signals that the
    /// "block" is finished.
    Empty,
}
