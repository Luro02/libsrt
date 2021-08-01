use core::str::FromStr;

use super::{parse_duration, Event, ParserError, State};
use crate::utils::{Lines, Span};

/// The [`EventParser`] parses each line in an `srt`-file and returns an
/// [`Event`] for it.
pub struct EventParser<'a> {
    lines: Lines<'a>,
    state: State,
}

impl<'a> From<&'a str> for EventParser<'a> {
    #[must_use]
    fn from(value: &'a str) -> Self {
        Self {
            lines: Lines::new(value),
            state: State::Counter,
        }
    }
}

impl<'a> Iterator for EventParser<'a> {
    type Item = Result<Event<'a>, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        let start_index = self.lines.index();

        if self.state == State::Empty {
            self.state.update();
            return Some(Ok(Event::Empty));
        }

        match &self.state {
            State::Counter => {
                let line = self.lines.next()?;

                // check for empty lines before the counter
                if line.is_empty() {
                    return Some(Ok(Event::Empty));
                }

                self.state.update();

                let line = line.trim();
                Some(
                    usize::from_str(&line)
                        .map(Event::Counter)
                        .map_err(|e| ParserError::parse_int_error(e, line.range())),
                )
            }
            State::Duration => {
                self.state.update();

                let line = self.lines.next()?;
                // TODO: maybe this also needs to split for " -> " (I saw this in some files)

                if let (start_string, Some(end_string)) = line.split_once(" --> ") {
                    let start = match parse_duration(start_string) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };

                    let end = match parse_duration(end_string) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };

                    Some(Ok(Event::Duration { start, end }))
                } else {
                    Some(Err(ParserError::invalid_duration(line.range())))
                }
            }
            State::Text => {
                self.state = State::Empty;
                let mut range_end = None;

                while let Some(line) = self.lines.next() {
                    if line.is_empty() {
                        break;
                    } else {
                        range_end = line.span().map(|span| span.end());
                    }
                }

                // TODO: is the range correct?
                range_end.map(|end| {
                    Ok(Event::Text({
                        let (string, span) = self.lines.get(start_index..end).unwrap().into_parts();

                        (string, span.start()..).into()
                    }))
                })
            }
            State::Empty => {
                self.state.update();
                Some(Ok(Event::Empty))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use core::time::Duration;

    #[test]
    fn test_parser_single_text() {
        let string = concat!(
            "1\n",
            "00:00:06,500 --> 00:00:09,000\n",
            "Single line of Text\n",
            "\n",
        );

        let mut parser = EventParser::from(string);

        assert_eq!(parser.next(), Some(Ok(Event::Counter(1))));
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Duration {
                start: Duration::from_secs_f64(6.5),
                end: Duration::from_secs(9)
            }))
        );
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Text(("Single line of Text", 32..).into())))
        );
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parser_multi_text() {
        let string = concat!(
            "1\n",
            "00:00:06,500 --> 00:00:09,000\n",
            "First Line of Text\nSecond Line of Text\n",
            "\n",
        );

        let mut parser = EventParser::from(string);

        assert_eq!(parser.next(), Some(Ok(Event::Counter(1))));
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Duration {
                start: Duration::from_secs_f64(6.5),
                end: Duration::from_secs(9)
            }))
        );
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Text(
                ("First Line of Text\nSecond Line of Text", 32..).into()
            )))
        );
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parser_multi_text_multi() {
        let string = concat!(
            "1\n",
            "00:00:06,500 --> 00:00:09,000\n",
            "First Line of Text\nSecond Line of Text\r\n",
            "\n",
            "2\n",
            "00:00:09,500 --> 00:00:11,000\n",
            "Third Line of Text\nFourth Line of Text"
        );

        let mut parser = EventParser::from(string);

        assert_eq!(parser.next(), Some(Ok(Event::Counter(1))));
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Duration {
                start: Duration::from_secs_f64(6.5),
                end: Duration::from_secs(9)
            }))
        );
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Text(
                ("First Line of Text\nSecond Line of Text", 32..).into()
            )))
        );
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));

        assert_eq!(parser.next(), Some(Ok(Event::Counter(2))));
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Duration {
                start: Duration::from_secs_f64(9.5),
                end: Duration::from_secs(11)
            }))
        );
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Text(
                ("Third Line of Text\nFourth Line of Text", 105..).into()
            )))
        );
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_with_additional_space() {
        let string = concat!(
            "\n",
            "1\n",
            "00:00:06,500 --> 00:00:09,000\n",
            "First Line of Text\nSecond Line of Text\r\n",
            "\n",
            "\n",
            "2\n",
            "00:00:09,500 --> 00:00:11,000\n",
            "Third Line of Text\nFourth Line of Text\n",
            "\n"
        );

        let mut parser = EventParser::from(string);

        assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        assert_eq!(parser.next(), Some(Ok(Event::Counter(1))));
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Duration {
                start: Duration::from_secs_f64(6.5),
                end: Duration::from_secs(9)
            }))
        );
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Text(
                ("First Line of Text\nSecond Line of Text", 33..).into()
            )))
        );
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));

        assert_eq!(parser.next(), Some(Ok(Event::Counter(2))));
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Duration {
                start: Duration::from_secs_f64(9.5),
                end: Duration::from_secs(11)
            }))
        );
        assert_eq!(
            parser.next(),
            Some(Ok(Event::Text(
                ("Third Line of Text\nFourth Line of Text", 107..).into()
            )))
        );
        assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        // TODO: the parser ignores additional empty lines and returns None, instead of
        //       the Event::Empty. Is this fine? (it is at the end of the input anyway)
        //
        // assert_eq!(parser.next(), Some(Ok(Event::Empty)));
        assert_eq!(parser.next(), None);
    }
}
