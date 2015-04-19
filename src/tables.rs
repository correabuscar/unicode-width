// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NOTE: The following code was generated by "scripts/unicode.py", do not edit directly

#![allow(missing_docs, non_upper_case_globals, non_snake_case)]

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-width is based on.
pub const UNICODE_VERSION: (u64, u64, u64) = (7, 0, 0);

pub mod charwidth {
    #[cfg(feature = "no_std")]
    use core::option::Option::{self, Some, None};
    #[cfg(feature = "no_std")]
    use core::slice::SliceExt;
    #[cfg(feature = "no_std")]
    use core::result::Result::{Ok, Err};

    #[inline]
    fn bsearch_range_value_table(c: char, is_cjk: bool, r: &'static [(char, char, u8, u8)]) -> u8 {
        #[cfg(feature = "no_std")]
        use core::cmp::Ordering::{Equal, Less, Greater};
        #[cfg(not(feature = "no_std"))]
        use std::cmp::Ordering::{Equal, Less, Greater};
        match r.binary_search_by(|&(lo, hi, _, _)| {
            if lo <= c && c <= hi { Equal }
            else if hi < c { Less }
            else { Greater }
        }) {
            Ok(idx) => {
                let (_, _, r_ncjk, r_cjk) = r[idx];
                if is_cjk { r_cjk } else { r_ncjk }
            }
            Err(_) => 1
        }
    }

    #[inline]
    pub fn width(c: char, is_cjk: bool) -> Option<usize> {
        match c as usize {
            _c @ 0 => Some(0),          // null is zero width
            cu if cu < 0x20 => None,    // control sequences have no width
            cu if cu < 0x7F => Some(1), // ASCII
            cu if cu < 0xA0 => None,    // more control sequences
            _ => Some(bsearch_range_value_table(c, is_cjk, charwidth_table) as usize)
        }
    }

    // character width table. Based on Markus Kuhn's free wcwidth() implementation,
    //     http://www.cl.cam.ac.uk/~mgk25/ucs/wcwidth.c
    const charwidth_table: &'static [(char, char, u8, u8)] = &[
        ('\u{a1}', '\u{a1}', 1, 2), ('\u{a4}', '\u{a4}', 1, 2), ('\u{a7}', '\u{a8}', 1, 2),
        ('\u{aa}', '\u{aa}', 1, 2), ('\u{ae}', '\u{ae}', 1, 2), ('\u{b0}', '\u{b4}', 1, 2),
        ('\u{b6}', '\u{ba}', 1, 2), ('\u{bc}', '\u{bf}', 1, 2), ('\u{c6}', '\u{c6}', 1, 2),
        ('\u{d0}', '\u{d0}', 1, 2), ('\u{d7}', '\u{d8}', 1, 2), ('\u{de}', '\u{e1}', 1, 2),
        ('\u{e6}', '\u{e6}', 1, 2), ('\u{e8}', '\u{ea}', 1, 2), ('\u{ec}', '\u{ed}', 1, 2),
        ('\u{f0}', '\u{f0}', 1, 2), ('\u{f2}', '\u{f3}', 1, 2), ('\u{f7}', '\u{fa}', 1, 2),
        ('\u{fc}', '\u{fc}', 1, 2), ('\u{fe}', '\u{fe}', 1, 2), ('\u{101}', '\u{101}', 1, 2),
        ('\u{111}', '\u{111}', 1, 2), ('\u{113}', '\u{113}', 1, 2), ('\u{11b}', '\u{11b}', 1, 2),
        ('\u{126}', '\u{127}', 1, 2), ('\u{12b}', '\u{12b}', 1, 2), ('\u{131}', '\u{133}', 1, 2),
        ('\u{138}', '\u{138}', 1, 2), ('\u{13f}', '\u{142}', 1, 2), ('\u{144}', '\u{144}', 1, 2),
        ('\u{148}', '\u{14b}', 1, 2), ('\u{14d}', '\u{14d}', 1, 2), ('\u{152}', '\u{153}', 1, 2),
        ('\u{166}', '\u{167}', 1, 2), ('\u{16b}', '\u{16b}', 1, 2), ('\u{1ce}', '\u{1ce}', 1, 2),
        ('\u{1d0}', '\u{1d0}', 1, 2), ('\u{1d2}', '\u{1d2}', 1, 2), ('\u{1d4}', '\u{1d4}', 1, 2),
        ('\u{1d6}', '\u{1d6}', 1, 2), ('\u{1d8}', '\u{1d8}', 1, 2), ('\u{1da}', '\u{1da}', 1, 2),
        ('\u{1dc}', '\u{1dc}', 1, 2), ('\u{251}', '\u{251}', 1, 2), ('\u{261}', '\u{261}', 1, 2),
        ('\u{2c4}', '\u{2c4}', 1, 2), ('\u{2c7}', '\u{2c7}', 1, 2), ('\u{2c9}', '\u{2cb}', 1, 2),
        ('\u{2cd}', '\u{2cd}', 1, 2), ('\u{2d0}', '\u{2d0}', 1, 2), ('\u{2d8}', '\u{2db}', 1, 2),
        ('\u{2dd}', '\u{2dd}', 1, 2), ('\u{2df}', '\u{2df}', 1, 2), ('\u{300}', '\u{36f}', 0, 0),
        ('\u{391}', '\u{3a1}', 1, 2), ('\u{3a3}', '\u{3a9}', 1, 2), ('\u{3b1}', '\u{3c1}', 1, 2),
        ('\u{3c3}', '\u{3c9}', 1, 2), ('\u{401}', '\u{401}', 1, 2), ('\u{410}', '\u{44f}', 1, 2),
        ('\u{451}', '\u{451}', 1, 2), ('\u{483}', '\u{489}', 0, 0), ('\u{591}', '\u{5bd}', 0, 0),
        ('\u{5bf}', '\u{5bf}', 0, 0), ('\u{5c1}', '\u{5c2}', 0, 0), ('\u{5c4}', '\u{5c5}', 0, 0),
        ('\u{5c7}', '\u{5c7}', 0, 0), ('\u{600}', '\u{605}', 0, 0), ('\u{610}', '\u{61a}', 0, 0),
        ('\u{61c}', '\u{61c}', 0, 0), ('\u{64b}', '\u{65f}', 0, 0), ('\u{670}', '\u{670}', 0, 0),
        ('\u{6d6}', '\u{6dd}', 0, 0), ('\u{6df}', '\u{6e4}', 0, 0), ('\u{6e7}', '\u{6e8}', 0, 0),
        ('\u{6ea}', '\u{6ed}', 0, 0), ('\u{70f}', '\u{70f}', 0, 0), ('\u{711}', '\u{711}', 0, 0),
        ('\u{730}', '\u{74a}', 0, 0), ('\u{7a6}', '\u{7b0}', 0, 0), ('\u{7eb}', '\u{7f3}', 0, 0),
        ('\u{816}', '\u{819}', 0, 0), ('\u{81b}', '\u{823}', 0, 0), ('\u{825}', '\u{827}', 0, 0),
        ('\u{829}', '\u{82d}', 0, 0), ('\u{859}', '\u{85b}', 0, 0), ('\u{8e4}', '\u{902}', 0, 0),
        ('\u{93a}', '\u{93a}', 0, 0), ('\u{93c}', '\u{93c}', 0, 0), ('\u{941}', '\u{948}', 0, 0),
        ('\u{94d}', '\u{94d}', 0, 0), ('\u{951}', '\u{957}', 0, 0), ('\u{962}', '\u{963}', 0, 0),
        ('\u{981}', '\u{981}', 0, 0), ('\u{9bc}', '\u{9bc}', 0, 0), ('\u{9c1}', '\u{9c4}', 0, 0),
        ('\u{9cd}', '\u{9cd}', 0, 0), ('\u{9e2}', '\u{9e3}', 0, 0), ('\u{a01}', '\u{a02}', 0, 0),
        ('\u{a3c}', '\u{a3c}', 0, 0), ('\u{a41}', '\u{a42}', 0, 0), ('\u{a47}', '\u{a48}', 0, 0),
        ('\u{a4b}', '\u{a4d}', 0, 0), ('\u{a51}', '\u{a51}', 0, 0), ('\u{a70}', '\u{a71}', 0, 0),
        ('\u{a75}', '\u{a75}', 0, 0), ('\u{a81}', '\u{a82}', 0, 0), ('\u{abc}', '\u{abc}', 0, 0),
        ('\u{ac1}', '\u{ac5}', 0, 0), ('\u{ac7}', '\u{ac8}', 0, 0), ('\u{acd}', '\u{acd}', 0, 0),
        ('\u{ae2}', '\u{ae3}', 0, 0), ('\u{b01}', '\u{b01}', 0, 0), ('\u{b3c}', '\u{b3c}', 0, 0),
        ('\u{b3f}', '\u{b3f}', 0, 0), ('\u{b41}', '\u{b44}', 0, 0), ('\u{b4d}', '\u{b4d}', 0, 0),
        ('\u{b56}', '\u{b56}', 0, 0), ('\u{b62}', '\u{b63}', 0, 0), ('\u{b82}', '\u{b82}', 0, 0),
        ('\u{bc0}', '\u{bc0}', 0, 0), ('\u{bcd}', '\u{bcd}', 0, 0), ('\u{c00}', '\u{c00}', 0, 0),
        ('\u{c3e}', '\u{c40}', 0, 0), ('\u{c46}', '\u{c48}', 0, 0), ('\u{c4a}', '\u{c4d}', 0, 0),
        ('\u{c55}', '\u{c56}', 0, 0), ('\u{c62}', '\u{c63}', 0, 0), ('\u{c81}', '\u{c81}', 0, 0),
        ('\u{cbc}', '\u{cbc}', 0, 0), ('\u{cbf}', '\u{cbf}', 0, 0), ('\u{cc6}', '\u{cc6}', 0, 0),
        ('\u{ccc}', '\u{ccd}', 0, 0), ('\u{ce2}', '\u{ce3}', 0, 0), ('\u{d01}', '\u{d01}', 0, 0),
        ('\u{d41}', '\u{d44}', 0, 0), ('\u{d4d}', '\u{d4d}', 0, 0), ('\u{d62}', '\u{d63}', 0, 0),
        ('\u{dca}', '\u{dca}', 0, 0), ('\u{dd2}', '\u{dd4}', 0, 0), ('\u{dd6}', '\u{dd6}', 0, 0),
        ('\u{e31}', '\u{e31}', 0, 0), ('\u{e34}', '\u{e3a}', 0, 0), ('\u{e47}', '\u{e4e}', 0, 0),
        ('\u{eb1}', '\u{eb1}', 0, 0), ('\u{eb4}', '\u{eb9}', 0, 0), ('\u{ebb}', '\u{ebc}', 0, 0),
        ('\u{ec8}', '\u{ecd}', 0, 0), ('\u{f18}', '\u{f19}', 0, 0), ('\u{f35}', '\u{f35}', 0, 0),
        ('\u{f37}', '\u{f37}', 0, 0), ('\u{f39}', '\u{f39}', 0, 0), ('\u{f71}', '\u{f7e}', 0, 0),
        ('\u{f80}', '\u{f84}', 0, 0), ('\u{f86}', '\u{f87}', 0, 0), ('\u{f8d}', '\u{f97}', 0, 0),
        ('\u{f99}', '\u{fbc}', 0, 0), ('\u{fc6}', '\u{fc6}', 0, 0), ('\u{102d}', '\u{1030}', 0, 0),
        ('\u{1032}', '\u{1037}', 0, 0), ('\u{1039}', '\u{103a}', 0, 0), ('\u{103d}', '\u{103e}', 0,
        0), ('\u{1058}', '\u{1059}', 0, 0), ('\u{105e}', '\u{1060}', 0, 0), ('\u{1071}', '\u{1074}',
        0, 0), ('\u{1082}', '\u{1082}', 0, 0), ('\u{1085}', '\u{1086}', 0, 0), ('\u{108d}',
        '\u{108d}', 0, 0), ('\u{109d}', '\u{109d}', 0, 0), ('\u{1100}', '\u{115f}', 2, 2),
        ('\u{1160}', '\u{11ff}', 0, 0), ('\u{135d}', '\u{135f}', 0, 0), ('\u{1712}', '\u{1714}', 0,
        0), ('\u{1732}', '\u{1734}', 0, 0), ('\u{1752}', '\u{1753}', 0, 0), ('\u{1772}', '\u{1773}',
        0, 0), ('\u{17b4}', '\u{17b5}', 0, 0), ('\u{17b7}', '\u{17bd}', 0, 0), ('\u{17c6}',
        '\u{17c6}', 0, 0), ('\u{17c9}', '\u{17d3}', 0, 0), ('\u{17dd}', '\u{17dd}', 0, 0),
        ('\u{180b}', '\u{180e}', 0, 0), ('\u{18a9}', '\u{18a9}', 0, 0), ('\u{1920}', '\u{1922}', 0,
        0), ('\u{1927}', '\u{1928}', 0, 0), ('\u{1932}', '\u{1932}', 0, 0), ('\u{1939}', '\u{193b}',
        0, 0), ('\u{1a17}', '\u{1a18}', 0, 0), ('\u{1a1b}', '\u{1a1b}', 0, 0), ('\u{1a56}',
        '\u{1a56}', 0, 0), ('\u{1a58}', '\u{1a5e}', 0, 0), ('\u{1a60}', '\u{1a60}', 0, 0),
        ('\u{1a62}', '\u{1a62}', 0, 0), ('\u{1a65}', '\u{1a6c}', 0, 0), ('\u{1a73}', '\u{1a7c}', 0,
        0), ('\u{1a7f}', '\u{1a7f}', 0, 0), ('\u{1ab0}', '\u{1abe}', 0, 0), ('\u{1b00}', '\u{1b03}',
        0, 0), ('\u{1b34}', '\u{1b34}', 0, 0), ('\u{1b36}', '\u{1b3a}', 0, 0), ('\u{1b3c}',
        '\u{1b3c}', 0, 0), ('\u{1b42}', '\u{1b42}', 0, 0), ('\u{1b6b}', '\u{1b73}', 0, 0),
        ('\u{1b80}', '\u{1b81}', 0, 0), ('\u{1ba2}', '\u{1ba5}', 0, 0), ('\u{1ba8}', '\u{1ba9}', 0,
        0), ('\u{1bab}', '\u{1bad}', 0, 0), ('\u{1be6}', '\u{1be6}', 0, 0), ('\u{1be8}', '\u{1be9}',
        0, 0), ('\u{1bed}', '\u{1bed}', 0, 0), ('\u{1bef}', '\u{1bf1}', 0, 0), ('\u{1c2c}',
        '\u{1c33}', 0, 0), ('\u{1c36}', '\u{1c37}', 0, 0), ('\u{1cd0}', '\u{1cd2}', 0, 0),
        ('\u{1cd4}', '\u{1ce0}', 0, 0), ('\u{1ce2}', '\u{1ce8}', 0, 0), ('\u{1ced}', '\u{1ced}', 0,
        0), ('\u{1cf4}', '\u{1cf4}', 0, 0), ('\u{1cf8}', '\u{1cf9}', 0, 0), ('\u{1dc0}', '\u{1df5}',
        0, 0), ('\u{1dfc}', '\u{1dff}', 0, 0), ('\u{200b}', '\u{200f}', 0, 0), ('\u{2010}',
        '\u{2010}', 1, 2), ('\u{2013}', '\u{2016}', 1, 2), ('\u{2018}', '\u{2019}', 1, 2),
        ('\u{201c}', '\u{201d}', 1, 2), ('\u{2020}', '\u{2022}', 1, 2), ('\u{2024}', '\u{2027}', 1,
        2), ('\u{202a}', '\u{202e}', 0, 0), ('\u{2030}', '\u{2030}', 1, 2), ('\u{2032}', '\u{2033}',
        1, 2), ('\u{2035}', '\u{2035}', 1, 2), ('\u{203b}', '\u{203b}', 1, 2), ('\u{203e}',
        '\u{203e}', 1, 2), ('\u{2060}', '\u{2064}', 0, 0), ('\u{2066}', '\u{206f}', 0, 0),
        ('\u{2074}', '\u{2074}', 1, 2), ('\u{207f}', '\u{207f}', 1, 2), ('\u{2081}', '\u{2084}', 1,
        2), ('\u{20ac}', '\u{20ac}', 1, 2), ('\u{20d0}', '\u{20f0}', 0, 0), ('\u{2103}', '\u{2103}',
        1, 2), ('\u{2105}', '\u{2105}', 1, 2), ('\u{2109}', '\u{2109}', 1, 2), ('\u{2113}',
        '\u{2113}', 1, 2), ('\u{2116}', '\u{2116}', 1, 2), ('\u{2121}', '\u{2122}', 1, 2),
        ('\u{2126}', '\u{2126}', 1, 2), ('\u{212b}', '\u{212b}', 1, 2), ('\u{2153}', '\u{2154}', 1,
        2), ('\u{215b}', '\u{215e}', 1, 2), ('\u{2160}', '\u{216b}', 1, 2), ('\u{2170}', '\u{2179}',
        1, 2), ('\u{2189}', '\u{2189}', 1, 2), ('\u{2190}', '\u{2199}', 1, 2), ('\u{21b8}',
        '\u{21b9}', 1, 2), ('\u{21d2}', '\u{21d2}', 1, 2), ('\u{21d4}', '\u{21d4}', 1, 2),
        ('\u{21e7}', '\u{21e7}', 1, 2), ('\u{2200}', '\u{2200}', 1, 2), ('\u{2202}', '\u{2203}', 1,
        2), ('\u{2207}', '\u{2208}', 1, 2), ('\u{220b}', '\u{220b}', 1, 2), ('\u{220f}', '\u{220f}',
        1, 2), ('\u{2211}', '\u{2211}', 1, 2), ('\u{2215}', '\u{2215}', 1, 2), ('\u{221a}',
        '\u{221a}', 1, 2), ('\u{221d}', '\u{2220}', 1, 2), ('\u{2223}', '\u{2223}', 1, 2),
        ('\u{2225}', '\u{2225}', 1, 2), ('\u{2227}', '\u{222c}', 1, 2), ('\u{222e}', '\u{222e}', 1,
        2), ('\u{2234}', '\u{2237}', 1, 2), ('\u{223c}', '\u{223d}', 1, 2), ('\u{2248}', '\u{2248}',
        1, 2), ('\u{224c}', '\u{224c}', 1, 2), ('\u{2252}', '\u{2252}', 1, 2), ('\u{2260}',
        '\u{2261}', 1, 2), ('\u{2264}', '\u{2267}', 1, 2), ('\u{226a}', '\u{226b}', 1, 2),
        ('\u{226e}', '\u{226f}', 1, 2), ('\u{2282}', '\u{2283}', 1, 2), ('\u{2286}', '\u{2287}', 1,
        2), ('\u{2295}', '\u{2295}', 1, 2), ('\u{2299}', '\u{2299}', 1, 2), ('\u{22a5}', '\u{22a5}',
        1, 2), ('\u{22bf}', '\u{22bf}', 1, 2), ('\u{2312}', '\u{2312}', 1, 2), ('\u{2329}',
        '\u{232a}', 2, 2), ('\u{2460}', '\u{24e9}', 1, 2), ('\u{24eb}', '\u{254b}', 1, 2),
        ('\u{2550}', '\u{2573}', 1, 2), ('\u{2580}', '\u{258f}', 1, 2), ('\u{2592}', '\u{2595}', 1,
        2), ('\u{25a0}', '\u{25a1}', 1, 2), ('\u{25a3}', '\u{25a9}', 1, 2), ('\u{25b2}', '\u{25b3}',
        1, 2), ('\u{25b6}', '\u{25b7}', 1, 2), ('\u{25bc}', '\u{25bd}', 1, 2), ('\u{25c0}',
        '\u{25c1}', 1, 2), ('\u{25c6}', '\u{25c8}', 1, 2), ('\u{25cb}', '\u{25cb}', 1, 2),
        ('\u{25ce}', '\u{25d1}', 1, 2), ('\u{25e2}', '\u{25e5}', 1, 2), ('\u{25ef}', '\u{25ef}', 1,
        2), ('\u{2605}', '\u{2606}', 1, 2), ('\u{2609}', '\u{2609}', 1, 2), ('\u{260e}', '\u{260f}',
        1, 2), ('\u{2614}', '\u{2615}', 1, 2), ('\u{261c}', '\u{261c}', 1, 2), ('\u{261e}',
        '\u{261e}', 1, 2), ('\u{2640}', '\u{2640}', 1, 2), ('\u{2642}', '\u{2642}', 1, 2),
        ('\u{2660}', '\u{2661}', 1, 2), ('\u{2663}', '\u{2665}', 1, 2), ('\u{2667}', '\u{266a}', 1,
        2), ('\u{266c}', '\u{266d}', 1, 2), ('\u{266f}', '\u{266f}', 1, 2), ('\u{269e}', '\u{269f}',
        1, 2), ('\u{26be}', '\u{26bf}', 1, 2), ('\u{26c4}', '\u{26cd}', 1, 2), ('\u{26cf}',
        '\u{26e1}', 1, 2), ('\u{26e3}', '\u{26e3}', 1, 2), ('\u{26e8}', '\u{26ff}', 1, 2),
        ('\u{273d}', '\u{273d}', 1, 2), ('\u{2757}', '\u{2757}', 1, 2), ('\u{2776}', '\u{277f}', 1,
        2), ('\u{2b55}', '\u{2b59}', 1, 2), ('\u{2cef}', '\u{2cf1}', 0, 0), ('\u{2d7f}', '\u{2d7f}',
        0, 0), ('\u{2de0}', '\u{2dff}', 0, 0), ('\u{2e80}', '\u{2e99}', 2, 2), ('\u{2e9b}',
        '\u{2ef3}', 2, 2), ('\u{2f00}', '\u{2fd5}', 2, 2), ('\u{2ff0}', '\u{2ffb}', 2, 2),
        ('\u{3000}', '\u{3029}', 2, 2), ('\u{302a}', '\u{302d}', 0, 0), ('\u{302e}', '\u{303e}', 2,
        2), ('\u{3041}', '\u{3096}', 2, 2), ('\u{3099}', '\u{309a}', 0, 0), ('\u{309b}', '\u{30ff}',
        2, 2), ('\u{3105}', '\u{312d}', 2, 2), ('\u{3131}', '\u{318e}', 2, 2), ('\u{3190}',
        '\u{31ba}', 2, 2), ('\u{31c0}', '\u{31e3}', 2, 2), ('\u{31f0}', '\u{321e}', 2, 2),
        ('\u{3220}', '\u{3247}', 2, 2), ('\u{3248}', '\u{324f}', 1, 2), ('\u{3250}', '\u{32fe}', 2,
        2), ('\u{3300}', '\u{4dbf}', 2, 2), ('\u{4e00}', '\u{a48c}', 2, 2), ('\u{a490}', '\u{a4c6}',
        2, 2), ('\u{a66f}', '\u{a672}', 0, 0), ('\u{a674}', '\u{a67d}', 0, 0), ('\u{a69f}',
        '\u{a69f}', 0, 0), ('\u{a6f0}', '\u{a6f1}', 0, 0), ('\u{a802}', '\u{a802}', 0, 0),
        ('\u{a806}', '\u{a806}', 0, 0), ('\u{a80b}', '\u{a80b}', 0, 0), ('\u{a825}', '\u{a826}', 0,
        0), ('\u{a8c4}', '\u{a8c4}', 0, 0), ('\u{a8e0}', '\u{a8f1}', 0, 0), ('\u{a926}', '\u{a92d}',
        0, 0), ('\u{a947}', '\u{a951}', 0, 0), ('\u{a960}', '\u{a97c}', 2, 2), ('\u{a980}',
        '\u{a982}', 0, 0), ('\u{a9b3}', '\u{a9b3}', 0, 0), ('\u{a9b6}', '\u{a9b9}', 0, 0),
        ('\u{a9bc}', '\u{a9bc}', 0, 0), ('\u{a9e5}', '\u{a9e5}', 0, 0), ('\u{aa29}', '\u{aa2e}', 0,
        0), ('\u{aa31}', '\u{aa32}', 0, 0), ('\u{aa35}', '\u{aa36}', 0, 0), ('\u{aa43}', '\u{aa43}',
        0, 0), ('\u{aa4c}', '\u{aa4c}', 0, 0), ('\u{aa7c}', '\u{aa7c}', 0, 0), ('\u{aab0}',
        '\u{aab0}', 0, 0), ('\u{aab2}', '\u{aab4}', 0, 0), ('\u{aab7}', '\u{aab8}', 0, 0),
        ('\u{aabe}', '\u{aabf}', 0, 0), ('\u{aac1}', '\u{aac1}', 0, 0), ('\u{aaec}', '\u{aaed}', 0,
        0), ('\u{aaf6}', '\u{aaf6}', 0, 0), ('\u{abe5}', '\u{abe5}', 0, 0), ('\u{abe8}', '\u{abe8}',
        0, 0), ('\u{abed}', '\u{abed}', 0, 0), ('\u{ac00}', '\u{d7a3}', 2, 2), ('\u{e000}',
        '\u{f8ff}', 1, 2), ('\u{f900}', '\u{faff}', 2, 2), ('\u{fb1e}', '\u{fb1e}', 0, 0),
        ('\u{fe00}', '\u{fe0f}', 0, 0), ('\u{fe10}', '\u{fe19}', 2, 2), ('\u{fe20}', '\u{fe2d}', 0,
        0), ('\u{fe30}', '\u{fe52}', 2, 2), ('\u{fe54}', '\u{fe66}', 2, 2), ('\u{fe68}', '\u{fe6b}',
        2, 2), ('\u{feff}', '\u{feff}', 0, 0), ('\u{ff01}', '\u{ff60}', 2, 2), ('\u{ffe0}',
        '\u{ffe6}', 2, 2), ('\u{fff9}', '\u{fffb}', 0, 0), ('\u{fffd}', '\u{fffd}', 1, 2),
        ('\u{101fd}', '\u{101fd}', 0, 0), ('\u{102e0}', '\u{102e0}', 0, 0), ('\u{10376}',
        '\u{1037a}', 0, 0), ('\u{10a01}', '\u{10a03}', 0, 0), ('\u{10a05}', '\u{10a06}', 0, 0),
        ('\u{10a0c}', '\u{10a0f}', 0, 0), ('\u{10a38}', '\u{10a3a}', 0, 0), ('\u{10a3f}',
        '\u{10a3f}', 0, 0), ('\u{10ae5}', '\u{10ae6}', 0, 0), ('\u{11001}', '\u{11001}', 0, 0),
        ('\u{11038}', '\u{11046}', 0, 0), ('\u{1107f}', '\u{11081}', 0, 0), ('\u{110b3}',
        '\u{110b6}', 0, 0), ('\u{110b9}', '\u{110ba}', 0, 0), ('\u{110bd}', '\u{110bd}', 0, 0),
        ('\u{11100}', '\u{11102}', 0, 0), ('\u{11127}', '\u{1112b}', 0, 0), ('\u{1112d}',
        '\u{11134}', 0, 0), ('\u{11173}', '\u{11173}', 0, 0), ('\u{11180}', '\u{11181}', 0, 0),
        ('\u{111b6}', '\u{111be}', 0, 0), ('\u{1122f}', '\u{11231}', 0, 0), ('\u{11234}',
        '\u{11234}', 0, 0), ('\u{11236}', '\u{11237}', 0, 0), ('\u{112df}', '\u{112df}', 0, 0),
        ('\u{112e3}', '\u{112ea}', 0, 0), ('\u{11301}', '\u{11301}', 0, 0), ('\u{1133c}',
        '\u{1133c}', 0, 0), ('\u{11340}', '\u{11340}', 0, 0), ('\u{11366}', '\u{1136c}', 0, 0),
        ('\u{11370}', '\u{11374}', 0, 0), ('\u{114b3}', '\u{114b8}', 0, 0), ('\u{114ba}',
        '\u{114ba}', 0, 0), ('\u{114bf}', '\u{114c0}', 0, 0), ('\u{114c2}', '\u{114c3}', 0, 0),
        ('\u{115b2}', '\u{115b5}', 0, 0), ('\u{115bc}', '\u{115bd}', 0, 0), ('\u{115bf}',
        '\u{115c0}', 0, 0), ('\u{11633}', '\u{1163a}', 0, 0), ('\u{1163d}', '\u{1163d}', 0, 0),
        ('\u{1163f}', '\u{11640}', 0, 0), ('\u{116ab}', '\u{116ab}', 0, 0), ('\u{116ad}',
        '\u{116ad}', 0, 0), ('\u{116b0}', '\u{116b5}', 0, 0), ('\u{116b7}', '\u{116b7}', 0, 0),
        ('\u{16af0}', '\u{16af4}', 0, 0), ('\u{16b30}', '\u{16b36}', 0, 0), ('\u{16f8f}',
        '\u{16f92}', 0, 0), ('\u{1b000}', '\u{1b001}', 2, 2), ('\u{1bc9d}', '\u{1bc9e}', 0, 0),
        ('\u{1bca0}', '\u{1bca3}', 0, 0), ('\u{1d167}', '\u{1d169}', 0, 0), ('\u{1d173}',
        '\u{1d182}', 0, 0), ('\u{1d185}', '\u{1d18b}', 0, 0), ('\u{1d1aa}', '\u{1d1ad}', 0, 0),
        ('\u{1d242}', '\u{1d244}', 0, 0), ('\u{1e8d0}', '\u{1e8d6}', 0, 0), ('\u{1f100}',
        '\u{1f10a}', 1, 2), ('\u{1f110}', '\u{1f12d}', 1, 2), ('\u{1f130}', '\u{1f169}', 1, 2),
        ('\u{1f170}', '\u{1f19a}', 1, 2), ('\u{1f200}', '\u{1f202}', 2, 2), ('\u{1f210}',
        '\u{1f23a}', 2, 2), ('\u{1f240}', '\u{1f248}', 2, 2), ('\u{1f250}', '\u{1f251}', 2, 2),
        ('\u{20000}', '\u{2fffd}', 2, 2), ('\u{30000}', '\u{3fffd}', 2, 2), ('\u{e0001}',
        '\u{e0001}', 0, 0), ('\u{e0020}', '\u{e007f}', 0, 0), ('\u{e0100}', '\u{e01ef}', 0, 0),
        ('\u{f0000}', '\u{ffffd}', 1, 2), ('\u{100000}', '\u{10fffd}', 1, 2)
    ];

}

