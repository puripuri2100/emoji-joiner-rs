use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use emoji_joiner::{emoji_joiner_with_emoji_data, make_emoji_data_14_0};

use fancy_regex;
use regex;

const EMOJI_STR_NORMAL_1: &str = "🧑‍🤝‍🧑👭👨‍👩‍👧‍👦";
const EMOJI_STR_BREAK_1: &str = "\u{1F9D1}\u{1F3FC}\u{200D}\u{2695}\u{FE0E}";
const EMOJI_STR_NORMAL_2: &str = "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}";
const EMOJI_STR_BREAK_2: &str = "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{0021}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}";

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
const EMOJI_PARSE_REGEX_CODE_UNICODE: &str = r"\p{RI}\p{RI}|\p{Emoji}(\p{EMod}|\x{FE0F}\x{20E3}?|[\x{E0020}-\x{E007E}]+\x{E007F})?(\x{200D}\p{Emoji}(\p{EMod}|\x{FE0F}\x{20E3}?|[\x{E0020}-\x{E007E}]+\x{E007F})?)*";

// Copyright Twitter Inc. Licensed under MIT
// https://github.com/twitter/twemoji-parser/blob/master/LICENSE.md
// https://github.com/twitter/twemoji-parser/blob/a97ef3994e4b88316812926844d51c296e889f76/src/lib/regex.js
const EMOJI_PARSE_REGEX_CODE_TWITTER: &str = r"(?:\u{1F468}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F9D1}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F9D1}[\u{1F3FC}-\u{1F3FF}]|\u{1F9D1}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F9D1}[\u{1F3FB}\u{1F3FD}-\u{1F3FF}]|\u{1F9D1}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F9D1}[\u{1F3FB}\u{1F3FC}\u{1F3FE}\u{1F3FF}]|\u{1F9D1}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FD}\u{1F3FF}]|\u{1F9D1}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FE}]|\u{1F468}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FB}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FC}-\u{1F3FF}]|\u{1F468}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FC}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}\u{1F3FD}-\u{1F3FF}]|\u{1F468}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FD}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}\u{1F3FC}\u{1F3FE}\u{1F3FF}]|\u{1F468}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FE}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FD}\u{1F3FF}]|\u{1F468}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{1F3FF}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FE}]|\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FB}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FC}-\u{1F3FF}]|\u{1F469}\u{1F3FB}\u{200D}\u{1F91D}\u{200D}\u{1F469}[\u{1F3FC}-\u{1F3FF}]|\u{1F469}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FC}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}\u{1F3FD}-\u{1F3FF}]|\u{1F469}\u{1F3FC}\u{200D}\u{1F91D}\u{200D}\u{1F469}[\u{1F3FB}\u{1F3FD}-\u{1F3FF}]|\u{1F469}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FD}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}\u{1F3FC}\u{1F3FE}\u{1F3FF}]|\u{1F469}\u{1F3FD}\u{200D}\u{1F91D}\u{200D}\u{1F469}[\u{1F3FB}\u{1F3FC}\u{1F3FE}\u{1F3FF}]|\u{1F469}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FE}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FD}\u{1F3FF}]|\u{1F469}\u{1F3FE}\u{200D}\u{1F91D}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FD}\u{1F3FF}]|\u{1F469}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FF}]|\u{1F469}\u{1F3FF}\u{200D}\u{1F91D}\u{200D}\u{1F468}[\u{1F3FB}-\u{1F3FE}]|\u{1F469}\u{1F3FF}\u{200D}\u{1F91D}\u{200D}\u{1F469}[\u{1F3FB}-\u{1F3FE}]|\u{1F9D1}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F9D1}[\u{1F3FC}-\u{1F3FF}]|\u{1F9D1}\u{1F3FB}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FF}]|\u{1F9D1}\u{1F3FC}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F9D1}[\u{1F3FB}\u{1F3FD}-\u{1F3FF}]|\u{1F9D1}\u{1F3FC}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FF}]|\u{1F9D1}\u{1F3FD}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F9D1}[\u{1F3FB}\u{1F3FC}\u{1F3FE}\u{1F3FF}]|\u{1F9D1}\u{1F3FD}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FF}]|\u{1F9D1}\u{1F3FE}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FD}\u{1F3FF}]|\u{1F9D1}\u{1F3FE}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FF}]|\u{1F9D1}\u{1F3FF}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FE}]|\u{1F9D1}\u{1F3FF}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}[\u{1F3FB}-\u{1F3FF}]|\u{1F468}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}\u{1F468}|\u{1F469}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F48B}\u{200D}[\u{1F468}\u{1F469}]|\u{1F468}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}|\u{1F469}\u{200D}\u{2764}\u{FE0F}\u{200D}[\u{1F468}\u{1F469}]|\u{1F9D1}\u{200D}\u{1F91D}\u{200D}\u{1F9D1}|\u{1F46B}[\u{1F3FB}-\u{1F3FF}]|\u{1F46C}[\u{1F3FB}-\u{1F3FF}]|\u{1F46D}[\u{1F3FB}-\u{1F3FF}]|\u{1F48F}[\u{1F3FB}-\u{1F3FF}]|\u{1F491}[\u{1F3FB}-\u{1F3FF}]|[\u{1F46B}-\u{1F46D}\u{1F48F}\u{1F491}])|(?:[\u{1F468}\u{1F469}]|\u{1F9D1})(?:[\u{1F3FB}-\u{1F3FF}])?\u{200D}(?:\u{2695}\u{FE0F}|\u{2696}\u{FE0F}|\u{2708}\u{FE0F}|[\u{1F33E}\u{1F373}\u{1F37C}\u{1F384}\u{1F393}\u{1F3A4}\u{1F3A8}\u{1F3EB}\u{1F3ED}]|[\u{1F4BB}\u{1F4BC}\u{1F527}\u{1F52C}\u{1F680}\u{1F692}]|[\u{1F9AF}-\u{1F9B3}\u{1F9BC}\u{1F9BD}])|(?:[\u{1F3CB}\u{1F3CC}]|[\u{1F574}\u{1F575}]|\u{26F9})((?:[\u{1F3FB}-\u{1F3FF}]|\u{FE0F})\u{200D}[\u{2640}\u{2642}]\u{FE0F})|(?:[\u{1F3C3}\u{1F3C4}\u{1F3CA}]|[\u{1F46E}\u{1F470}\u{1F471}\u{1F473}\u{1F477}\u{1F481}\u{1F482}\u{1F486}\u{1F487}\u{1F645}-\u{1F647}\u{1F64B}\u{1F64D}\u{1F64E}\u{1F6A3}\u{1F6B4}-\u{1F6B6}]|[\u{1F926}\u{1F935}\u{1F937}-\u{1F939}\u{1F93D}\u{1F93E}\u{1F9B8}\u{1F9B9}\u{1F9CD}-\u{1F9CF}\u{1F9D4}\u{1F9D6}-\u{1F9DD}])(?:[\u{1F3FB}-\u{1F3FF}])?\u{200D}[\u{2640}\u{2642}]\u{FE0F}|(?:\u{1F468}\u{200D}\u{1F468}\u{200D}\u{1F466}\u{200D}\u{1F466}|\u{1F468}\u{200D}\u{1F468}\u{200D}\u{1F467}\u{200D}[\u{1F466}\u{1F467}]|\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466}\u{200D}\u{1F466}|\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}[\u{1F466}\u{1F467}]|\u{1F469}\u{200D}\u{1F469}\u{200D}\u{1F466}\u{200D}\u{1F466}|\u{1F469}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}[\u{1F466}\u{1F467}]|\u{1F468}\u{200D}\u{1F466}\u{200D}\u{1F466}|\u{1F468}\u{200D}\u{1F467}\u{200D}[\u{1F466}\u{1F467}]|\u{1F468}\u{200D}\u{1F468}\u{200D}[\u{1F466}\u{1F467}]|\u{1F468}\u{200D}\u{1F469}\u{200D}[\u{1F466}\u{1F467}]|\u{1F469}\u{200D}\u{1F466}\u{200D}\u{1F466}|\u{1F469}\u{200D}\u{1F467}\u{200D}[\u{1F466}\u{1F467}]|\u{1F469}\u{200D}\u{1F469}\u{200D}[\u{1F466}\u{1F467}]|\u{1F3F3}\u{FE0F}\u{200D}\u{26A7}\u{FE0F}|\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}|\u{1F636}\u{200D}\u{1F32B}\u{FE0F}|\u{2764}\u{FE0F}\u{200D}\u{1F525}|\u{2764}\u{FE0F}\u{200D}\u{1FA79}|\u{1F3F4}\u{200D}\u{2620}\u{FE0F}|\u{1F415}\u{200D}\u{1F9BA}|\u{1F43B}\u{200D}\u{2744}\u{FE0F}|\u{1F441}\u{200D}\u{1F5E8}|\u{1F468}\u{200D}[\u{1F466}\u{1F467}]|\u{1F469}\u{200D}[\u{1F466}\u{1F467}]|\u{1F46F}\u{200D}\u{2640}\u{FE0F}|\u{1F46F}\u{200D}\u{2642}\u{FE0F}|\u{1F62E}\u{200D}\u{1F4A8}|\u{1F635}\u{200D}\u{1F4AB}|\u{1F93C}\u{200D}\u{2640}\u{FE0F}|\u{1F93C}\u{200D}\u{2642}\u{FE0F}|\u{1F9DE}\u{200D}\u{2640}\u{FE0F}|\u{1F9DE}\u{200D}\u{2642}\u{FE0F}|\u{1F9DF}\u{200D}\u{2640}\u{FE0F}|\u{1F9DF}\u{200D}\u{2642}\u{FE0F}|\u{1F408}\u{200D}\u{2B1B})|[#*0-9]\u{FE0F}?\u{20E3}|(?:[©®\u{2122}\u{265F}]\u{FE0F})|(?:[\u{1F004}\u{1F170}\u{1F171}\u{1F17E}\u{1F17F}\u{1F202}\u{1F21A}\u{1F22F}\u{1F237}\u{1F321}\u{1F324}-\u{1F32C}\u{1F336}\u{1F37D}\u{1F396}\u{1F397}\u{1F399}-\u{1F39B}\u{1F39E}\u{1F39F}\u{1F3CD}\u{1F3CE}\u{1F3D4}-\u{1F3DF}\u{1F3F3}\u{1F3F5}\u{1F3F7}]|[\u{1F43F}\u{1F441}\u{1F4FD}\u{1F549}\u{1F54A}\u{1F56F}\u{1F570}\u{1F573}\u{1F576}-\u{1F579}\u{1F587}\u{1F58A}-\u{1F58D}\u{1F5A5}\u{1F5A8}\u{1F5B1}\u{1F5B2}\u{1F5BC}\u{1F5C2}-\u{1F5C4}\u{1F5D1}-\u{1F5D3}\u{1F5DC}-\u{1F5DE}\u{1F5E1}\u{1F5E3}\u{1F5E8}\u{1F5EF}\u{1F5F3}\u{1F5FA}\u{1F6CB}\u{1F6CD}-\u{1F6CF}\u{1F6E0}-\u{1F6E5}\u{1F6E9}\u{1F6F0}\u{1F6F3}]|[\u{203C}\u{2049}\u{2139}\u{2194}-\u{2199}\u{21A9}\u{21AA}\u{231A}\u{231B}\u{2328}\u{23CF}\u{23ED}-\u{23EF}\u{23F1}\u{23F2}\u{23F8}-\u{23FA}\u{24C2}\u{25AA}\u{25AB}\u{25B6}\u{25C0}\u{25FB}-\u{25FE}\u{2600}-\u{2604}\u{260E}\u{2611}\u{2614}\u{2615}\u{2618}\u{2620}\u{2622}\u{2623}\u{2626}\u{262A}\u{262E}\u{262F}\u{2638}-\u{263A}\u{2640}\u{2642}\u{2648}-\u{2653}\u{2660}\u{2663}\u{2665}\u{2666}\u{2668}\u{267B}\u{267F}\u{2692}-\u{2697}\u{2699}\u{269B}\u{269C}\u{26A0}\u{26A1}\u{26A7}\u{26AA}\u{26AB}\u{26B0}\u{26B1}\u{26BD}\u{26BE}\u{26C4}\u{26C5}\u{26C8}\u{26CF}\u{26D1}\u{26D3}\u{26D4}\u{26E9}\u{26EA}\u{26F0}-\u{26F5}\u{26F8}\u{26FA}\u{26FD}\u{2702}\u{2708}\u{2709}\u{270F}\u{2712}\u{2714}\u{2716}\u{271D}\u{2721}\u{2733}\u{2734}\u{2744}\u{2747}\u{2757}\u{2763}\u{2764}\u{27A1}\u{2934}\u{2935}\u{2B05}-\u{2B07}\u{2B1B}\u{2B1C}\u{2B50}\u{2B55}\u{3030}\u{303D}\u{3297}\u{3299}])(?:\u{FE0F}|(?!\u{FE0E}))|(?:(?:[\u{1F3CB}\u{1F3CC}]|[\u{1F574}\u{1F575}\u{1F590}]|[\u{261D}\u{26F7}\u{26F9}\u{270C}\u{270D}])(?:\u{FE0F}|(?!\u{FE0E}))|(?:[\u{1F385}\u{1F3C2}-\u{1F3C4}\u{1F3C7}\u{1F3CA}]|[\u{1F442}\u{1F443}\u{1F446}-\u{1F450}\u{1F466}-\u{1F469}\u{1F46E}\u{1F470}-\u{1F478}\u{1F47C}\u{1F481}-\u{1F483}\u{1F485}-\u{1F487}\u{1F4AA}\u{1F57A}\u{1F595}\u{1F596}\u{1F645}-\u{1F647}\u{1F64B}-\u{1F64F}\u{1F6A3}\u{1F6B4}-\u{1F6B6}\u{1F6C0}\u{1F6CC}]|[\u{1F90C}\u{1F90F}\u{1F918}-\u{1F91C}\u{1F91E}\u{1F91F}\u{1F926}\u{1F930}-\u{1F939}\u{1F93D}\u{1F93E}\u{1F977}\u{1F9B5}\u{1F9B6}\u{1F9B8}\u{1F9B9}\u{1F9BB}\u{1F9CD}-\u{1F9CF}\u{1F9D1}-\u{1F9DD}]|[\u{270A}\u{270B}]))(?:[\u{1F3FB}-\u{1F3FF}])?|(?:\u{1F3F4}\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}|\u{1F3F4}\u{E0067}\u{E0062}\u{E0073}\u{E0063}\u{E0074}\u{E007F}|\u{1F3F4}\u{E0067}\u{E0062}\u{E0077}\u{E006C}\u{E0073}\u{E007F}|\u{1F1E6}[\u{1F1E8}-\u{1F1EC}\u{1F1EE}\u{1F1F1}\u{1F1F2}\u{1F1F4}\u{1F1F6}-\u{1F1FA}\u{1F1FC}\u{1F1FD}\u{1F1FF}]|\u{1F1E7}[\u{1F1E6}\u{1F1E7}\u{1F1E9}-\u{1F1EF}\u{1F1F1}-\u{1F1F4}\u{1F1F6}-\u{1F1F9}\u{1F1FB}\u{1F1FC}\u{1F1FE}\u{1F1FF}]|\u{1F1E8}[\u{1F1E6}\u{1F1E8}\u{1F1E9}\u{1F1EB}-\u{1F1EE}\u{1F1F0}-\u{1F1F5}\u{1F1F7}\u{1F1FA}-\u{1F1FF}]|\u{1F1E9}[\u{1F1EA}\u{1F1EC}\u{1F1EF}\u{1F1F0}\u{1F1F2}\u{1F1F4}\u{1F1FF}]|\u{1F1EA}[\u{1F1E6}\u{1F1E8}\u{1F1EA}\u{1F1EC}\u{1F1ED}\u{1F1F7}-\u{1F1FA}]|\u{1F1EB}[\u{1F1EE}-\u{1F1F0}\u{1F1F2}\u{1F1F4}\u{1F1F7}]|\u{1F1EC}[\u{1F1E6}\u{1F1E7}\u{1F1E9}-\u{1F1EE}\u{1F1F1}-\u{1F1F3}\u{1F1F5}-\u{1F1FA}\u{1F1FC}\u{1F1FE}]|\u{1F1ED}[\u{1F1F0}\u{1F1F2}\u{1F1F3}\u{1F1F7}\u{1F1F9}\u{1F1FA}]|\u{1F1EE}[\u{1F1E8}-\u{1F1EA}\u{1F1F1}-\u{1F1F4}\u{1F1F6}-\u{1F1F9}]|\u{1F1EF}[\u{1F1EA}\u{1F1F2}\u{1F1F4}\u{1F1F5}]|\u{1F1F0}[\u{1F1EA}\u{1F1EC}-\u{1F1EE}\u{1F1F2}\u{1F1F3}\u{1F1F5}\u{1F1F7}\u{1F1FC}\u{1F1FE}\u{1F1FF}]|\u{1F1F1}[\u{1F1E6}-\u{1F1E8}\u{1F1EE}\u{1F1F0}\u{1F1F7}-\u{1F1FB}\u{1F1FE}]|\u{1F1F2}[\u{1F1E6}\u{1F1E8}-\u{1F1ED}\u{1F1F0}-\u{1F1FF}]|\u{1F1F3}[\u{1F1E6}\u{1F1E8}\u{1F1EA}-\u{1F1EC}\u{1F1EE}\u{1F1F1}\u{1F1F4}\u{1F1F5}\u{1F1F7}\u{1F1FA}\u{1F1FF}]|\u{1F1F4}\u{1F1F2}|\u{1F1F5}[\u{1F1E6}\u{1F1EA}-\u{1F1ED}\u{1F1F0}-\u{1F1F3}\u{1F1F7}-\u{1F1F9}\u{1F1FC}\u{1F1FE}]|\u{1F1F6}\u{1F1E6}|\u{1F1F7}[\u{1F1EA}\u{1F1F4}\u{1F1F8}\u{1F1FA}\u{1F1FC}]|\u{1F1F8}[\u{1F1E6}-\u{1F1EA}\u{1F1EC}-\u{1F1F4}\u{1F1F7}-\u{1F1F9}\u{1F1FB}\u{1F1FD}-\u{1F1FF}]|\u{1F1F9}[\u{1F1E6}\u{1F1E8}\u{1F1E9}\u{1F1EB}-\u{1F1ED}\u{1F1EF}-\u{1F1F4}\u{1F1F7}\u{1F1F9}\u{1F1FB}\u{1F1FC}\u{1F1FF}]|\u{1F1FA}[\u{1F1E6}\u{1F1EC}\u{1F1F2}\u{1F1F3}\u{1F1F8}\u{1F1FE}\u{1F1FF}]|\u{1F1FB}[\u{1F1E6}\u{1F1E8}\u{1F1EA}\u{1F1EC}\u{1F1EE}\u{1F1F3}\u{1F1FA}]|\u{1F1FC}[\u{1F1EB}\u{1F1F8}]|\u{1F1FD}\u{1F1F0}|\u{1F1FE}[\u{1F1EA}\u{1F1F9}]|\u{1F1FF}[\u{1F1E6}\u{1F1F2}\u{1F1FC}]|[\u{1F0CF}\u{1F18E}\u{1F191}-\u{1F19A}\u{1F1E6}-\u{1F1FF}\u{1F201}\u{1F232}-\u{1F236}\u{1F238}-\u{1F23A}\u{1F250}\u{1F251}\u{1F300}-\u{1F320}\u{1F32D}-\u{1F335}\u{1F337}-\u{1F37C}\u{1F37E}-\u{1F384}\u{1F386}-\u{1F393}\u{1F3A0}-\u{1F3C1}\u{1F3C5}\u{1F3C6}\u{1F3C8}\u{1F3C9}\u{1F3CF}-\u{1F3D3}\u{1F3E0}-\u{1F3F0}\u{1F3F4}\u{1F3F8}-\u{1F3FF}]|[\u{1F400}-\u{1F43E}\u{1F440}\u{1F444}\u{1F445}\u{1F451}-\u{1F465}\u{1F46A}\u{1F46F}\u{1F479}-\u{1F47B}\u{1F47D}-\u{1F480}\u{1F484}\u{1F488}-\u{1F48E}\u{1F490}\u{1F492}-\u{1F4A9}\u{1F4AB}-\u{1F4FC}\u{1F4FF}-\u{1F53D}\u{1F54B}-\u{1F54E}\u{1F550}-\u{1F567}\u{1F5A4}\u{1F5FB}-\u{1F644}\u{1F648}-\u{1F64A}\u{1F680}-\u{1F6A2}\u{1F6A4}-\u{1F6B3}\u{1F6B7}-\u{1F6BF}\u{1F6C1}-\u{1F6C5}\u{1F6D0}-\u{1F6D2}\u{1F6D5}-\u{1F6D7}\u{1F6EB}\u{1F6EC}\u{1F6F4}-\u{1F6FC}\u{1F7E0}-\u{1F7EB}]|[\u{1F90D}\u{1F90E}\u{1F910}-\u{1F917}\u{1F91D}\u{1F920}-\u{1F925}\u{1F927}-\u{1F92F}\u{1F93A}\u{1F93C}\u{1F93F}-\u{1F945}\u{1F947}-\u{1F976}\u{1F978}\u{1F97A}-\u{1F9B4}\u{1F9B7}\u{1F9BA}\u{1F9BC}-\u{1F9CB}\u{1F9D0}\u{1F9DE}-\u{1F9FF}\u{1FA70}-\u{1FA74}\u{1FA78}-\u{1FA7A}\u{1FA80}-\u{1FA86}\u{1FA90}-\u{1FAA8}\u{1FAB0}-\u{1FAB6}\u{1FAC0}-\u{1FAC2}\u{1FAD0}-\u{1FAD6}]|[\u{23E9}-\u{23EC}\u{23F0}\u{23F3}\u{267E}\u{26CE}\u{2705}\u{2728}\u{274C}\u{274E}\u{2753}-\u{2755}\u{2795}-\u{2797}\u{27B0}\u{27BF}\u{E50A}])|\u{FE0F}";

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
fn parse_emoji_with_regex_unicode(re: &regex::Regex, str: &str) -> Vec<String> {
  let mut v = Vec::new();
  let bytes = str.as_bytes();
  let mut start_pos = 0;
  for mat in re.find_iter(str) {
    if start_pos < mat.start() {
      if let Some(l) = bytes.get(start_pos..mat.start()) {
        let mut d = String::from_utf8(l.to_vec())
          .unwrap()
          .chars()
          .map(|c| c.to_string())
          .collect::<Vec<String>>();
        v.append(&mut d);
      }
    }
    v.push(mat.as_str().to_string());
    start_pos = mat.end();
  }
  if start_pos < bytes.len() {
    if let Some(l) = bytes.get(start_pos..bytes.len()) {
      let mut d = String::from_utf8(l.to_vec())
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
      v.append(&mut d);
    }
  }
  v
}

fn make_parser_regex_unicode_benchmark(c: &mut Criterion) {
  c.bench_function("make_parser_regex_unicode", |b| {
    b.iter(|| regex::Regex::new(EMOJI_PARSE_REGEX_CODE_UNICODE).unwrap());
  });
}

fn regex_parse_unicode_benchmark(c: &mut Criterion) {
  c.bench_function("regex_parse_unicode", |b| {
    let emoji_re = regex::Regex::new(EMOJI_PARSE_REGEX_CODE_UNICODE).unwrap();
    b.iter(|| parse_emoji_with_regex_unicode(&emoji_re, black_box(EMOJI_STR_NORMAL_1)));
    // 普通の結合絵文字の解析1
    b.iter(|| parse_emoji_with_regex_unicode(&emoji_re, black_box(EMOJI_STR_NORMAL_1)));
    // 壊れる絵文字の解析1
    b.iter(|| parse_emoji_with_regex_unicode(&emoji_re, black_box(EMOJI_STR_BREAK_1)));
    // 普通の結合絵文字の解析2
    b.iter(|| parse_emoji_with_regex_unicode(&emoji_re, black_box(EMOJI_STR_NORMAL_2)));
    // 壊れる絵文字の解析2
    b.iter(|| parse_emoji_with_regex_unicode(&emoji_re, black_box(EMOJI_STR_BREAK_2)));
  });
}

// twitter社が公開している、正確な解析を行える正規表現を使う
// https://github.com/twitter/twemoji-parser/blob/HEAD/src/lib/regex.js
fn parse_emoji_with_regex_twitter(re: &fancy_regex::Regex, str: &str) -> Vec<String> {
  let mut v = Vec::new();
  let bytes = str.as_bytes();
  let mut start_pos = 0;
  for mat in re.find_iter(str) {
    let mat = mat.unwrap();
    if start_pos < mat.start() {
      if let Some(l) = bytes.get(start_pos..mat.start()) {
        let mut d = String::from_utf8(l.to_vec())
          .unwrap()
          .chars()
          .map(|c| c.to_string())
          .collect::<Vec<String>>();
        v.append(&mut d);
      }
    }
    v.push(mat.as_str().to_string());
    start_pos = mat.end();
  }
  if start_pos < bytes.len() {
    if let Some(l) = bytes.get(start_pos..bytes.len()) {
      let mut d = String::from_utf8(l.to_vec())
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
      v.append(&mut d);
    }
  }
  v
}

fn make_parser_regex_twitter_benchmark(c: &mut Criterion) {
  c.bench_function("make_parser_regex_twitter", |b| {
    b.iter(|| fancy_regex::Regex::new(EMOJI_PARSE_REGEX_CODE_TWITTER).unwrap());
  });
}

fn regex_parse_twitter_benchmark(c: &mut Criterion) {
  c.bench_function("regex_parse_twitter", |b| {
    let emoji_re = fancy_regex::Regex::new(EMOJI_PARSE_REGEX_CODE_TWITTER).unwrap();
    b.iter(|| parse_emoji_with_regex_twitter(&emoji_re, black_box(EMOJI_STR_NORMAL_1)));
    // 普通の結合絵文字の解析1
    b.iter(|| parse_emoji_with_regex_twitter(&emoji_re, black_box(EMOJI_STR_NORMAL_1)));
    // 壊れる絵文字の解析1
    b.iter(|| parse_emoji_with_regex_twitter(&emoji_re, black_box(EMOJI_STR_BREAK_1)));
    // 普通の結合絵文字の解析2
    b.iter(|| parse_emoji_with_regex_twitter(&emoji_re, black_box(EMOJI_STR_NORMAL_2)));
    // 壊れる絵文字の解析2
    b.iter(|| parse_emoji_with_regex_twitter(&emoji_re, black_box(EMOJI_STR_BREAK_2)));
  });
}

fn parsers_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("emoji parser");
  let emoji_data = make_emoji_data_14_0();
  let emoji_re = fancy_regex::Regex::new(EMOJI_PARSE_REGEX_CODE_TWITTER).unwrap();
  for s in [
    EMOJI_STR_NORMAL_1,
    EMOJI_STR_NORMAL_2,
    EMOJI_STR_BREAK_1,
    EMOJI_STR_BREAK_2,
  ]
  .iter()
  {
    group.bench_with_input(BenchmarkId::new("emoji-joiner", s), s, |b, s| {
      b.iter(|| emoji_joiner_with_emoji_data(&emoji_data, black_box(s)))
    });
    group.bench_with_input(BenchmarkId::new("regex-twitter", s), s, |b, s| {
      b.iter(|| parse_emoji_with_regex_twitter(&emoji_re, black_box(s)))
    });
  }
  group.finish();
}

criterion_group!(benches_make_emoji_data_14_0, make_emoji_data_14_0_benchmark);

criterion_group!(
  benches_emoji_joiner_with_emoji_data,
  emoji_joiner_with_emoji_data_benchmark
);

criterion_group!(
  benches_make_parser_regex_unicode,
  make_parser_regex_unicode_benchmark
);

criterion_group!(benches_regex_parse_unicode, regex_parse_unicode_benchmark);

criterion_group!(
  benches_make_parser_regex_twitter,
  make_parser_regex_twitter_benchmark
);

criterion_group!(benches_regex_parse_twitter, regex_parse_twitter_benchmark);

criterion_group!(bench_parsers, parsers_benchmark);

criterion_main!(
  benches_make_emoji_data_14_0,
  benches_emoji_joiner_with_emoji_data,
  benches_make_parser_regex_unicode,
  benches_regex_parse_unicode,
  benches_make_parser_regex_twitter,
  benches_regex_parse_twitter,
  bench_parsers,
);
