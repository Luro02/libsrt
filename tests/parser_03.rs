/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{subtitle::SubtitleInit, Text};

    use core::time::Duration;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_inline_optimization() {
        assert_eq!(
            ::core::mem::size_of::<Subtitle<'_>>(),
            ::core::mem::size_of::<Option<Subtitle<'_>>>()
        );
    }

    #[test]
    fn test_from_str() {
        let input = concat!(
            "1\n",
            "00:00:01,600 --> 00:00:04,200\n",
            "English (US)\n",
            "\n",
            "2\n",
            "00:00:05,900 --> 00:00:07,999\n",
            "This is a subtitle in American English\n",
            "\n",
            "3\n",
            "00:00:10,000 --> 00:00:14,000\n",
            "Adding subtitles is very easy to do"
        );

        assert_eq!(
            SubRipTitle::try_from(input),
            Ok(SubRipTitle::builder()
                .push_subtitle(
                    SubtitleInit {
                        counter: 1,
                        start: Duration::from_secs_f64(1.6),
                        duration: Duration::from_secs_f64(4.2 - 1.6),
                        text: Text::new_spanned("English (US)", 32),
                        ..SubtitleInit::default()
                    }
                    .init()
                    .unwrap()
                )
                .push_subtitle(
                    SubtitleInit {
                        counter: 2,
                        start: Duration::from_secs_f64(5.9),
                        // 7.999 - 5.900
                        duration: Duration::from_secs(7) + Duration::from_millis(999)
                            - Duration::from_secs(5)
                            - Duration::from_millis(900),
                        text: Text::new_spanned("This is a subtitle in American English", 78),
                        ..SubtitleInit::default()
                    }
                    .init()
                    .unwrap()
                )
                .push_subtitle(
                    SubtitleInit {
                        counter: 3,
                        start: Duration::from_secs(10),
                        duration: Duration::from_secs(14 - 10),
                        text: Text::new_spanned("Adding subtitles is very easy to do", 150),
                        ..SubtitleInit::default()
                    }
                    .init()
                    .unwrap()
                )
                .build()
                .unwrap())
        );
    }
}

*/
