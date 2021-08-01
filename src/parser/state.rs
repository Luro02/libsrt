#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum State {
    Counter,
    Duration,
    Text,
    Empty,
}

impl State {
    #[inline]
    pub fn update(&mut self) {
        match self {
            Self::Counter => *self = Self::Duration,
            Self::Duration => *self = Self::Text,
            Self::Text | Self::Empty => *self = Self::Counter,
        }
    }
}
