///
/// @package Rubtle-Lib
///
/// @file Util functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

#[allow(unused_macros)]
macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0")
            as *const str
            as *const [::std::os::raw::c_char]
            as *const ::std::os::raw::c_char
    )
}

#[allow(unused_macros)]
macro_rules! i8str {
    ($($b:expr),*) => ([$($b as i8),*, 0])
}

#[allow(unused_macros)]
macro_rules! hidden_i8str {
    ($($b:expr),*) => (i8str!(-1, $($b as i8),*))
}