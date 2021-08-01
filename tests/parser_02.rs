use std::time::Duration;

use libsrt::{SubtitleInit, SubtitleIterator};
use pretty_assertions::assert_eq;

// the subtitle has been taken from: https://en.wikipedia.org/wiki/SubRip
const KOREAN_SUBTITLE: &str = concat!(
    "1\n",
    "00:03:17,440 --> 00:53:20,375\n",
    "칠레 한인회에서는  그동안 코로나-19로 인해서 빠트로나또 한인타운가가 5개월 동안 문을.\n",
    "정성기 칠레 한인회장님께서 방역 활동을 하고 계십니다.  방역 활동 구역은 만사노, 산타.\n",
    "황성남 한인회이사님께서도 방역 활동에 동참 해 주셨습니다.\n",
    "\n",
    "2\n",
    "00:54:20,476 --> 03:16:22,501\n",
    "까날 트레쎄 13번 칠레 방송국에서 빠트로나또 개장 현황을 취재하던 중에 한인회의.\n",
    "촬영이 끝난 후 인터뷰 요청이 있어서 정성기 한인회장님께서 인터뷰를 하게 되었습니다. \n",
    "금일 9월7일 월요일 밤 9시 뉴스시간에 촬영 및 인터뷰 내용이 방영 된다고 합니다.\n",
    "<ref></ref>우리 모두 코로나-19 퇴치 운동에 적극 동참하여 또 다시 자가격리로 되돌.\n",
    "\n",
);

macro_rules! assert_into_span {
    ($string:expr, $start:literal..) => {{
        let length = $string.len();
        assert_eq!(&KOREAN_SUBTITLE[$start..$start + length], $string);
        ($string, $start..).into()
    }};
}

#[test]
fn parse_korean_subtitle() {
    let mut parser = SubtitleIterator::from(KOREAN_SUBTITLE);

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 1,
            start: Duration::from_secs_f64(197.44),
            duration: Duration::from_secs_f64(3002.935),
            text: assert_into_span!(concat!(
                "칠레 한인회에서는  그동안 코로나-19로 인해서 빠트로나또 한인타운가가 5개월 동안 문을.\n",
                "정성기 칠레 한인회장님께서 방역 활동을 하고 계십니다.  방역 활동 구역은 만사노, 산타.\n",
                "황성남 한인회이사님께서도 방역 활동에 동참 해 주셨습니다."
            ), 32..),
            ..SubtitleInit::default()
        }.init().unwrap()))
    );

    assert_eq!(
        parser.next(),
        Some(Ok(SubtitleInit {
            counter: 2,
            start: Duration::from_secs_f64(3260.476),
            duration: Duration::from_secs_f64(8522.025),
            text: assert_into_span!(concat!(
                "까날 트레쎄 13번 칠레 방송국에서 빠트로나또 개장 현황을 취재하던 중에 한인회의.\n",
                "촬영이 끝난 후 인터뷰 요청이 있어서 정성기 한인회장님께서 인터뷰를 하게 되었습니다. \n",
                "금일 9월7일 월요일 밤 9시 뉴스시간에 촬영 및 인터뷰 내용이 방영 된다고 합니다.\n",
                "<ref></ref>우리 모두 코로나-19 퇴치 운동에 적극 동참하여 또 다시 자가격리로 되돌."
            ), 390..),
            ..SubtitleInit::default()
        }.init().unwrap()))
    );

    assert_eq!(parser.next(), None);
}
