// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Global initialization and retreival of command line arguments.
//!
//! On some platforms these are stored during runtime startup,
//! and on some they are retrieved from the system on demand.

#![allow(dead_code)] // runtime init functions not used during testing

use ffi::OsString;
use marker::PhantomData;
use vec;

/// One-time global initialization.
pub unsafe fn init(argc: isize, argv: *const *const u8) { imp::init(argc, argv) }

/// One-time global cleanup.
pub unsafe fn cleanup() { imp::cleanup() }

/// Returns the command line arguments
pub fn args() -> Args {
    imp::args()
}

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize { self.iter.len() }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> { self.iter.next_back() }
}

mod imp {
    use ffi::{CStr, OsString};
    use sys::ext::ffi::OsStringExt;
    use marker::PhantomData;
    use super::Args;
    use cell::UnsafeCell;
    use libc;

    static mut ARGS: UnsafeCell<Option<Vec<Vec<u8>>>> = UnsafeCell::new(None);

    pub unsafe fn init(argc: isize, argv: *const *const u8) {
        let args = (0..argc).map(|i| {
            CStr::from_ptr(*argv.offset(i) as *const libc::c_char).to_bytes().to_vec()
        }).collect();

        *ARGS.get() = Some(args);
    }

    pub unsafe fn cleanup() {
        *ARGS.get() = None;
    }

    pub fn args() -> Args {
        let bytes = clone().unwrap_or(Vec::new());
        let v: Vec<OsString> = bytes.into_iter().map(|v| {
            OsStringExt::from_vec(v)
        }).collect();
        Args { iter: v.into_iter(), _dont_send_or_sync_me: PhantomData }
    }

    fn clone() -> Option<Vec<Vec<u8>>> {
        unsafe {
            (*ARGS.get()).as_ref().cloned()
        }
    }
}
