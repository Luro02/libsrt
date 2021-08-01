use core::ops::RangeBounds;

use logical_pattern::OrPattern;

use super::pattern::PatternExt;
use super::Span;
use super::spanned::SplitTerminator;
use crate::utils::Spanned;

// TODO: implement basic traits
pub(crate) struct Lines<'a> {
    start: usize,
    iterator: SplitTerminator<'a, OrPattern<&'a str, char>>,
}

impl<'a> Lines<'a> {
    #[inline]
    #[must_use]
    pub fn new(string: &'a str) -> Self { Self::new_spanned(Spanned::new(string)) }

    #[inline]
    #[must_use]
    pub fn new_spanned(string: Spanned<&'a str>) -> Self {
        Self {
            iterator: string.split_terminator(PatternExt::or("\r\n", '\n')),
            start: string.span().map_or(0, Span::start),
        }
    }

    #[must_use]
    pub fn get<R: RangeBounds<usize>>(&self, range: R) -> Option<Spanned<&'a str>> {
        let spanned = Spanned::from((self.iterator.0.haystack(), self.start..));

        spanned.get(range)
    }

    #[inline]
    #[must_use]
    pub fn index(&self) -> usize { self.iterator.0.matcher.index() }
}

impl<'a> Iterator for Lines<'a> {
    type Item = Spanned<&'a str>;

    fn next(&mut self) -> Option<Self::Item> { Some(self.iterator.next()?) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lines_1() {
        let data = "\nMäry häd ä little lämb\n\r\nLittle lämb\n";
        let mut lines = Lines::new(data);

        assert_eq!("", &data[0..0]);
        assert_eq!(lines.next(), Some(("", 0..).into()));
        assert_eq!(lines.index(), 1);

        assert_eq!("Märy häd ä little lämb", &data[1..27]);
        assert_eq!(lines.next(), Some(("Märy häd ä little lämb", 1..).into()));
        assert_eq!(lines.index(), 28);

        assert_eq!("", &data[28..28]);
        assert_eq!(lines.next(), Some(("", 28..).into()));
        assert_eq!(lines.index(), 30);

        assert_eq!("Little lämb", &data[30..42]);
        assert_eq!(lines.next(), Some(("Little lämb", 30..).into()));
        assert_eq!(lines.index(), 43);

        assert_eq!(lines.next(), None);
        assert_eq!(lines.index(), 43);
    }

    #[test]
    fn test_lines_2() {
        let data = "\r\nMäry häd ä little lämb\n\nLittle lämb";
        let mut lines = Lines::new(data); // no trailing \n

        assert_eq!("", &data[0..0]);
        assert_eq!(lines.next(), Some(("", 0..).into()));
        assert_eq!(lines.index(), 2);

        assert_eq!("Märy häd ä little lämb", &data[2..28]);
        assert_eq!(lines.next(), Some(("Märy häd ä little lämb", 2..).into()));
        assert_eq!(lines.index(), 29);

        assert_eq!("", &data[29..29]);
        assert_eq!(lines.next(), Some(("", 29..).into()));
        assert_eq!(lines.index(), 30);

        assert_eq!("Little lämb", &data[30..42]);
        assert_eq!(lines.next(), Some(("Little lämb", 30..).into()));
        assert_eq!(lines.index(), 42);

        assert_eq!(lines.next(), None);
        assert_eq!(lines.index(), 42);
    }

    #[test]
    fn test_lines_3() {
        let data = concat!(
            //
            "1\n",
            "00:00:01,600 --> 00:00:04,200\n",
            "English (US)\n",
        );

        let mut lines = Lines::new(data);

        assert_eq!("1", &data[0..1]);
        assert_eq!(lines.next(), Some(("1", 0..).into()));
        assert_eq!(lines.index(), 2);

        assert_eq!("00:00:01,600 --> 00:00:04,200", &data[2..31]);
        assert_eq!(
            lines.next(),
            Some(("00:00:01,600 --> 00:00:04,200", 2..).into())
        );
        assert_eq!(lines.index(), 32);

        assert_eq!("English (US)", &data[32..44]);
        assert_eq!(lines.next(), Some(("English (US)", 32..).into()));
        assert_eq!(lines.index(), 45);

        assert_eq!(lines.next(), None);
        assert_eq!(lines.index(), 45);
    }

    #[test]
    fn test_lines_4() {
        let data = concat!(
            "\n",
            "1\n",
            "00:00:01,300 --> 00:00:08,240\n",
            "今天录制了专辑的第三首歌「Fluegel」\n",
            "\n",
        );

        let mut lines = Lines::new(data);

        assert_eq!("", &data[0..0]);
        assert_eq!(lines.next(), Some(("", 0..).into()));
        assert_eq!(lines.index(), 1);

        assert_eq!("1", &data[1..2]);
        assert_eq!(lines.next(), Some(("1", 1..).into()));
        assert_eq!(lines.index(), 3);

        assert_eq!("00:00:01,300 --> 00:00:08,240", &data[3..32]);
        assert_eq!(
            lines.next(),
            Some(("00:00:01,300 --> 00:00:08,240", 3..).into())
        );
        assert_eq!(lines.index(), 33);

        // NOTE: this string has a length of 49
        assert_eq!("今天录制了专辑的第三首歌「Fluegel」".len(), 49);
        assert_eq!("今天录制了专辑的第三首歌「Fluegel」", &data[33..82]);
        assert_eq!(
            lines.next(),
            Some(("今天录制了专辑的第三首歌「Fluegel」", 33..).into())
        );
        assert_eq!(lines.index(), 83);
    }

    #[test]
    fn test_lines_5() {
        // TODO: this should be tested with subtitle parsing (index function might be
        // broken)
        let data = concat!(
            "Маком исалок убитал едляет дает варасс. \n",
            "Хотори дение был в возмен стивое мо объекс к Иной'\n"
        );

        let mut lines = Lines::new(data);

        assert_eq!("Маком исалок убитал едляет дает варасс. ", &data[0..73]);
        assert_eq!(
            lines.next(),
            Some(("Маком исалок убитал едляет дает варасс. ", 0..).into())
        );
        assert_eq!(lines.index(), 74);

        assert_eq!(
            "Хотори дение был в возмен стивое мо объекс к Иной'",
            &data[74..164]
        );
        assert_eq!(
            lines.next(),
            Some(("Хотори дение был в возмен стивое мо объекс к Иной'", 74..).into())
        );
        assert_eq!(lines.index(), 165);

        assert_eq!(lines.next(), None);
        assert_eq!(lines.index(), 165);
    }
}
