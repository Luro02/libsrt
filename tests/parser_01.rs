use std::time::Duration;

use libsrt::{SubtitleInit, SubtitleIterator};
use pretty_assertions::assert_eq;

const CHINESE_SUBTITLE: &str = concat!(
    "\n",
    "1\n",
    "00:00:01,300 --> 00:00:08,240\n",
    "今天录制了专辑的第三首歌「Fluegel」\n",
    "\n",
    "2\n",
    "00:00:08,240 --> 00:00:12,400\n",
    "非常高卡路里的一首歌\n",
    "\n",
    "3\n",
    "00:00:12,400 --> 00:00:17,240\n",
    "非常有异域风情的很帅气的一首歌\n",
    "\n",
    "4\n",
    "00:00:17,240 --> 00:00:20,380\n",
    "唱歌的时候也很不容易\n",
    "\n",
    "5\n",
    "00:00:20,380 --> 00:00:25,740\n",
    "心情上也感觉自己简直瘦了一两千克\n",
    "\n",
    "6\n",
    "00:00:25,740 --> 00:00:29,020\n",
    "作为第三首曲子，是至今为止最让我措手不及的一首\n",
    "\n",
    "7\n",
    "00:00:29,020 --> 00:00:32,360\n",
    "接下来会有什么曲子我也有点小不安\n",
    "\n",
    "8\n",
    "00:00:32,360 --> 00:00:38,460\n",
    "今天给大家展示一下录音的时候我自己用的歌词卡\n",
    "\n",
    "9\n",
    "00:00:39,760 --> 00:00:41,420\n",
    "看得到吗\n",
    "\n",
    "10\n",
    "00:00:41,420 --> 00:00:45,200\n",
    "写了相当多的内容\n",
    "\n",
    "11\n",
    "00:00:45,200 --> 00:00:49,440\n",
    "这里写了\"美丽系萨满第15年\"\n",
    "\n",
    "12\n",
    "00:00:49,440 --> 00:00:53,000\n",
    "这里还有\"人类\"，\"简单\"，\"真实体验\"之类的\n",
    "\n",
    "13\n",
    "00:00:53,000 --> 00:00:55,860\n",
    "写了很多这样的迷之评论\n",
    "\n",
    "14\n",
    "00:00:55,860 --> 00:01:03,240\n",
    "这个\"美丽系萨满第15年\"也请务必听着歌去理解一下\n",
    "\n",
    "15\n",
    "00:01:03,240 --> 00:01:09,040\n",
    "今天也是用尽了全力很累了，感觉能睡的很香\n",
    "\n",
    "16\n",
    "00:01:09,040 --> 00:01:10,700\n",
    "晚安~\n",
    "\n",
);

macro_rules! assert_into_span {
    ($string:expr, $start:literal..) => {{
        let length = $string.len();
        assert_eq!(&CHINESE_SUBTITLE[$start..$start + length], $string);
        ($string, $start..).into()
    }};
}

#[test]
fn parse_chinese_subtitle() {
    let mut parser = SubtitleIterator::from(CHINESE_SUBTITLE);

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 1,
            start: Duration::from_secs_f64(1.3),
            duration: Duration::from_secs_f64(6.94),
            text: assert_into_span!("今天录制了专辑的第三首歌「Fluegel」", 33..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 2,
            start: Duration::from_secs_f64(8.24),
            duration: Duration::from_secs_f64(4.16),
            text: assert_into_span!("非常高卡路里的一首歌", 116..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 3,
            start: Duration::from_secs_f64(12.4),
            duration: Duration::from_secs_f64(4.84),
            text: assert_into_span!("非常有异域风情的很帅气的一首歌", 180..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 4,
            start: Duration::from_secs_f64(17.24),
            duration: Duration::from_secs_f64(3.14),
            text: assert_into_span!("唱歌的时候也很不容易", 259..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 5,
            start: Duration::from_secs_f64(20.38),
            duration: Duration::from_secs_f64(5.36),
            text: assert_into_span!("心情上也感觉自己简直瘦了一两千克", 323..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 6,
            start: Duration::from_secs_f64(25.74),
            duration: Duration::from_secs_f64(3.28),
            text: assert_into_span!("作为第三首曲子，是至今为止最让我措手不及的一首", 405..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 7,
            start: Duration::from_secs_f64(29.02),
            duration: Duration::from_secs_f64(3.34),
            text: assert_into_span!("接下来会有什么曲子我也有点小不安", 508..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 8,
            start: Duration::from_secs_f64(32.36),
            duration: Duration::from_secs_f64(6.1),
            text: assert_into_span!("今天给大家展示一下录音的时候我自己用的歌词卡", 590..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 9,
            start: Duration::from_secs_f64(39.76),
            duration: Duration::from_secs_f64(1.66),
            text: assert_into_span!("看得到吗", 690..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 10,
            start: Duration::from_secs_f64(41.42),
            duration: Duration::from_secs_f64(3.78),
            text: assert_into_span!("写了相当多的内容", 737..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 11,
            start: Duration::from_secs_f64(45.2),
            duration: Duration::from_secs_f64(4.24),
            text: assert_into_span!("这里写了\"美丽系萨满第15年\"", 796..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 12,
            start: Duration::from_secs_f64(49.44),
            duration: Duration::from_secs_f64(3.56),
            text: assert_into_span!("这里还有\"人类\"，\"简单\"，\"真实体验\"之类的", 868..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 13,
            start: Duration::from_secs(53),
            duration: Duration::from_secs_f64(2.86),
            text: assert_into_span!("写了很多这样的迷之评论", 960..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 14,
            start: Duration::from_secs_f64(55.86),
            duration: Duration::from_secs_f64(7.38),
            text: assert_into_span!("这个\"美丽系萨满第15年\"也请务必听着歌去理解一下", 1028..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 15,
            start: Duration::from_secs_f64(63.24),
            duration: Duration::from_secs_f64(5.8),
            text: assert_into_span!("今天也是用尽了全力很累了，感觉能睡的很香", 1130..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 16,
            start: Duration::from_secs_f64(69.04),
            duration: Duration::from_secs_f64(1.66),
            text: assert_into_span!("晚安~", 1225..),
            ..SubtitleInit::default()
        }
        .init()
        .unwrap()))
    );

    assert_eq!(parser.next(), None);
}
