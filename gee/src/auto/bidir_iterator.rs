// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7c3d3f5+)
// DO NOT EDIT

use crate::Iterator;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct BidirIterator(Interface<ffi::GeeBidirIterator>) @requires Iterator;

    match fn {
        get_type => || ffi::gee_bidir_iterator_get_type(),
    }
}

pub const NONE_BIDIR_ITERATOR: Option<&BidirIterator> = None;

pub trait BidirIteratorExt: 'static {
    #[doc(alias = "gee_bidir_iterator_previous")]
    fn previous() -> bool;

    #[doc(alias = "gee_bidir_iterator_has_previous")]
    fn has_previous() -> bool;

    #[doc(alias = "gee_bidir_iterator_first")]
    fn first() -> bool;

    #[doc(alias = "gee_bidir_iterator_last")]
    fn last() -> bool;
}

impl<O: IsA<BidirIterator>> BidirIteratorExt for O {
    fn previous() -> bool {
        unsafe {
            from_glib(ffi::gee_bidir_iterator_previous())
        }
    }

    fn has_previous() -> bool {
        unsafe {
            from_glib(ffi::gee_bidir_iterator_has_previous())
        }
    }

    fn first() -> bool {
        unsafe {
            from_glib(ffi::gee_bidir_iterator_first())
        }
    }

    fn last() -> bool {
        unsafe {
            from_glib(ffi::gee_bidir_iterator_last())
        }
    }
}

impl fmt::Display for BidirIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BidirIterator")
    }
}
