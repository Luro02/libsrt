use crate::parser::{Event, EventParser};
use crate::{Subtitle, SubtitleError, SubtitleInit};

#[must_use]
pub struct SubtitleIterator<'a> {
    parser: EventParser<'a>,
}

impl<'a> From<&'a str> for SubtitleIterator<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            parser: EventParser::from(value),
        }
    }
}

impl<'a> Iterator for SubtitleIterator<'a> {
    type Item = Result<Subtitle<'a>, SubtitleError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = None;
        let mut duration = None;
        let mut lines = None;

        while let Some(event) = self.parser.next() {
            // TODO: replace with something smarter
            let event = {
                match event {
                    Ok(value) => value,
                    Err(error) => return Some(Err(error.into())),
                }
            };

            match event {
                Event::Counter(c) => counter = Some(c),
                Event::Duration { start, end } => duration = Some((start, end)),
                Event::Text(text) => {
                    lines = Some(text);
                }
                Event::Empty => {
                    // skip empty lines in front of a subtitle
                    if counter.is_none() && duration.is_none() && lines.is_none() {
                        continue;
                    }

                    // check that all three parts of a subtitle are present:
                    if let (Some(counter), Some((start, end)), Some(text)) =
                        (counter, duration, lines)
                    {
                        let subtitle = SubtitleInit {
                            counter,
                            start,
                            duration: end - start,
                            text: text.into(),
                            ..SubtitleInit::default()
                        }
                        .init();

                        // TODO: simplify
                        return subtitle
                            .map(|value| Some(value))
                            .map_err(|error| SubtitleError::Init(error))
                            .transpose();
                    } else if counter.is_some() {
                        return Some(Err(SubtitleError::MissingCounter));
                    } else if duration.is_some() {
                        return Some(Err(SubtitleError::MissingDuration));
                    } else {
                        return Some(Err(SubtitleError::MissingText));
                    }
                }
            }
        }

        None
    }
}
/*
pub(crate) fn from_parser(parser: &mut Parser<'a>) -> Result<Self, Option<SubtitleError>> {

    }
*/
