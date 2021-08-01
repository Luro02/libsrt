use super::Span;

pub trait Spannable {
    fn span(&self) -> Span;
}

impl Spannable for str {
    fn span(&self) -> Span { Span::from(0..self.len()) }
}

impl<'a> Spannable for &'a str {
    fn span(&self) -> Span { Span::from(0..self.len()) }
}
