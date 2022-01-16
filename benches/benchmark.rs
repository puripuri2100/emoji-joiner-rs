#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use emoji_joiner::{emoji_joiner_with_emoji_data, make_emoji_data_14_0};

use regex::Regex;

const EMOJI_STR_NORMAL_1: &str = "🧑‍🤝‍🧑👭👨‍👩‍👧‍👦";
const EMOJI_STR_BREAK_1: &str = "\u{1F9D1}\u{1F3FC}\u{200D}\u{2695}\u{FE0E}";
const EMOJI_STR_NORMAL_2: &str = "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}";
const EMOJI_STR_BREAK_2: &str = "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{0023}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}";

// http://www.unicode.org/reports/tr51/#EBNF_and_Regex
// UNICODE, INC. LICENSE AGREEMENT - DATA FILES AND SOFTWARE
//
// See Terms of Use <https://www.unicode.org/copyright.html>
// for definitions of Unicode Inc.’s Data Files and Software.
//
// NOTICE TO USER: Carefully read the following legal agreement.
// BY DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING UNICODE INC.'S
// DATA FILES ("DATA FILES"), AND/OR SOFTWARE ("SOFTWARE"),
// YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
// TERMS AND CONDITIONS OF THIS AGREEMENT.
// IF YOU DO NOT AGREE, DO NOT DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE
// THE DATA FILES OR SOFTWARE.
//
// COPYRIGHT AND PERMISSION NOTICE
//
// Copyright © 1991-2022 Unicode, Inc. All rights reserved.
// Distributed under the Terms of Use in https://www.unicode.org/copyright.html.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of the Unicode data files and any associated documentation
// (the "Data Files") or Unicode software and any associated documentation
// (the "Software") to deal in the Data Files or Software
// without restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, and/or sell copies of
// the Data Files or Software, and to permit persons to whom the Data Files
// or Software are furnished to do so, provided that either
// (a) this copyright and permission notice appear with all copies
// of the Data Files or Software, or
// (b) this copyright and permission notice appear in associated
// Documentation.
//
// THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
// WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT OF THIRD PARTY RIGHTS.
// IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS
// NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL
// DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THE DATA FILES OR SOFTWARE.
//
// Except as contained in this notice, the name of a copyright holder
// shall not be used in advertising or otherwise to promote the sale,
// use or other dealings in these Data Files or Software without prior
// written authorization of the copyright holder.
const EMOJI_PARSE_REGEX_CODE: &str = r"\p{RI}\p{RI}|\p{Emoji}(\p{EMod}|\x{FE0F}\x{20E3}?|[\x{E0020}-\x{E007E}]+\x{E007F})?(\x{200D}\p{Emoji}(\p{EMod}|\x{FE0F}\x{20E3}?|[\x{E0020}-\x{E007E}]+\x{E007F})?)*";

fn make_emoji_data_14_0_benchmark(c: &mut Criterion) {
  c.bench_function("make_emoji_data_14_0", |b| {
    b.iter(|| make_emoji_data_14_0())
  });
}

fn emoji_joiner_with_emoji_data_benchmark(c: &mut Criterion) {
  c.bench_function("emoji_joiner_with_emoji_data with unicode 14.0", |b| {
    let emoji_data = make_emoji_data_14_0();
    // 普通の結合絵文字の解析1
    b.iter(|| emoji_joiner_with_emoji_data(&emoji_data, black_box(EMOJI_STR_NORMAL_1)));
    // 壊れる絵文字の解析1
    b.iter(|| emoji_joiner_with_emoji_data(&emoji_data, black_box(EMOJI_STR_BREAK_1)));
    // 普通の結合絵文字の解析2
    b.iter(|| emoji_joiner_with_emoji_data(&emoji_data, black_box(EMOJI_STR_NORMAL_2)));
    // 壊れる絵文字の解析2
    b.iter(|| emoji_joiner_with_emoji_data(&emoji_data, black_box(EMOJI_STR_BREAK_2)));
  });
}

// 速いが正確性とカスタマイズ性が犠牲になる
// 例えば
// "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F421}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}"
// を二文字に分割してしまう
// 正しくは（少なくとも14.0では）
// "\u{1F469}\u{1F3FB}\u{200D\u{2764}\u{FE0F}\u{200D}\u{1F421}\u{1F3FB}"
// という文字列は存在しない（"\u{1F421}"はフグの絵文字）
fn parse_emoji_with_regex(re: &Regex, str: &str) -> Vec<Vec<char>> {
  let mut v = Vec::new();
  let mut s = str.to_string();
  while !s.is_empty() {
    let str_bytes = s.as_bytes();
    match re.find(&s) {
      Some(m) => {
        v.push(m.as_str().chars().collect::<Vec<_>>());
        let last =
          String::from_utf8(str_bytes.iter().skip(m.end()).copied().collect::<Vec<_>>()).unwrap();
        s = last;
      }
      None => {
        let mut chars = s.chars();
        match chars.next() {
          None => break,
          Some(c) => v.push(vec![c]),
        }
        s = chars.collect::<String>();
      }
    }
  }
  v
}

fn make_parser_regex_benchmark(c: &mut Criterion) {
  c.bench_function("make_parser_regex", |b| {
    b.iter(|| Regex::new(EMOJI_PARSE_REGEX_CODE).unwrap());
  });
}

fn regex_parse_benchmark(c: &mut Criterion) {
  c.bench_function("regex_parse", |b| {
    let emoji_re = Regex::new(EMOJI_PARSE_REGEX_CODE).unwrap();
    b.iter(|| parse_emoji_with_regex(&emoji_re, black_box(EMOJI_STR_NORMAL_1)));
    // 普通の結合絵文字の解析1
    b.iter(|| parse_emoji_with_regex(&emoji_re, black_box(EMOJI_STR_NORMAL_1)));
    // 壊れる絵文字の解析1
    b.iter(|| parse_emoji_with_regex(&emoji_re, black_box(EMOJI_STR_BREAK_1)));
    // 普通の結合絵文字の解析2
    b.iter(|| parse_emoji_with_regex(&emoji_re, black_box(EMOJI_STR_NORMAL_2)));
    // 壊れる絵文字の解析2
    b.iter(|| parse_emoji_with_regex(&emoji_re, black_box(EMOJI_STR_BREAK_2)));
  });
}

criterion_group!(benches_make_emoji_data_14_0, make_emoji_data_14_0_benchmark);

criterion_group!(
  benches_emoji_joiner_with_emoji_data,
  emoji_joiner_with_emoji_data_benchmark
);

criterion_group!(benches_make_parser_regex, make_parser_regex_benchmark);

criterion_group!(benches_regex_parse, regex_parse_benchmark);

criterion_main!(
  benches_make_emoji_data_14_0,
  benches_emoji_joiner_with_emoji_data,
  benches_make_parser_regex,
  benches_regex_parse
);
