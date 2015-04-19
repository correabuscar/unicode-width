// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter;
use test::{self, Bencher};

use super::UnicodeWidthChar;

#[cfg(feature = "no_std")]
use std::prelude::v1::*;

#[bench]
fn cargo(b: &mut Bencher) {
    let string = iter::repeat('a').take(4096).collect::<String>();

    b.iter(|| {
        for c in string.chars() {
            test::black_box(UnicodeWidthChar::width(c));
        }
    });
}

#[bench]
fn stdlib(b: &mut Bencher) {
    let string = iter::repeat('a').take(4096).collect::<String>();

    b.iter(|| {
        for c in string.chars() {
            test::black_box(c.width(false));
        }
    });
}

#[bench]
fn simple_if(b: &mut Bencher) {
    let string = iter::repeat('a').take(4096).collect::<String>();

    b.iter(|| {
        for c in string.chars() {
            test::black_box(simple_width_if(c));
        }
    });
}

#[bench]
fn simple_match(b: &mut Bencher) {
    let string = iter::repeat('a').take(4096).collect::<String>();

    b.iter(|| {
        for c in string.chars() {
            test::black_box(simple_width_match(c));
        }
    });
}

#[inline]
fn simple_width_if(c: char) -> Option<usize> {
    let cu = c as u32;
    if cu < 127 {
        if cu > 31 {
            Some(1)
        } else if cu == 0 {
            Some(0)
        } else {
            None
        }
    } else {
        UnicodeWidthChar::width(c)
    }
}

#[inline]
fn simple_width_match(c: char) -> Option<usize> {
    match c as u32 {
        cu if cu == 0 => Some(0),
        cu if cu < 0x20 => None,
        cu if cu < 0x7f => Some(1),
        _ => UnicodeWidthChar::width(c)
    }
}

#[test]
fn test_str() {
    use super::UnicodeWidthStr;

    assert_eq!(UnicodeWidthStr::width("ｈｅｌｌｏ"), 10);
    assert_eq!("ｈｅｌｌｏ".width_cjk(), 10);
    assert_eq!(UnicodeWidthStr::width("\0\0\0\x01\x01"), 0);
    assert_eq!("\0\0\0\x01\x01".width_cjk(), 0);
    assert_eq!(UnicodeWidthStr::width(""), 0);
    assert_eq!("".width_cjk(), 0);
    assert_eq!(UnicodeWidthStr::width("\u{2081}\u{2082}\u{2083}\u{2084}"), 4);
    assert_eq!("\u{2081}\u{2082}\u{2083}\u{2084}".width_cjk(), 8);
}

#[test]
fn test_char() {
    use super::UnicodeWidthChar;
    #[cfg(feature = "no_std")]
    use core::option::Option::{Some, None};

    assert_eq!(UnicodeWidthChar::width('ｈ'), Some(2));
    assert_eq!('ｈ'.width_cjk(), Some(2));
    assert_eq!(UnicodeWidthChar::width('\x00'), Some(0));
    assert_eq!('\x00'.width_cjk(), Some(0));
    assert_eq!(UnicodeWidthChar::width('\x01'), None);
    assert_eq!('\x01'.width_cjk(), None);
    assert_eq!(UnicodeWidthChar::width('\u{2081}'), Some(1));
    assert_eq!('\u{2081}'.width_cjk(), Some(2));
}

#[test]
fn test_char2() {
    use super::UnicodeWidthChar;
    #[cfg(feature = "no_std")]
    use core::option::Option::{Some, None};

    assert_eq!(UnicodeWidthChar::width('\x00'),Some(0));
    assert_eq!('\x00'.width_cjk(),Some(0));

    assert_eq!(UnicodeWidthChar::width('\x0A'),None);
    assert_eq!('\x0A'.width_cjk(),None);

    assert_eq!(UnicodeWidthChar::width('w'),Some(1));
    assert_eq!('w'.width_cjk(),Some(1));

    assert_eq!(UnicodeWidthChar::width('ｈ'),Some(2));
    assert_eq!('ｈ'.width_cjk(),Some(2));

    assert_eq!(UnicodeWidthChar::width('\u{AD}'),Some(1));
    assert_eq!('\u{AD}'.width_cjk(),Some(1));

    assert_eq!(UnicodeWidthChar::width('\u{1160}'),Some(0));
    assert_eq!('\u{1160}'.width_cjk(),Some(0));

    assert_eq!(UnicodeWidthChar::width('\u{a1}'),Some(1));
    assert_eq!('\u{a1}'.width_cjk(),Some(2));

    assert_eq!(UnicodeWidthChar::width('\u{300}'),Some(0));
    assert_eq!('\u{300}'.width_cjk(),Some(0));
}
