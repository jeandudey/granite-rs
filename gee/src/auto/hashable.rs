// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7c3d3f5+)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Hashable(Interface<ffi::GeeHashable>);

    match fn {
        get_type => || ffi::gee_hashable_get_type(),
    }
}

pub const NONE_HASHABLE: Option<&Hashable> = None;

pub trait HashableExt: 'static {
    #[doc(alias = "gee_hashable_hash")]
    fn hash() -> u32;

    //#[doc(alias = "gee_hashable_equal_to")]
    //fn equal_to(object: /*Unimplemented*/Fundamental: Pointer) -> bool;
}

impl<O: IsA<Hashable>> HashableExt for O {
    fn hash() -> u32 {
        unsafe {
            ffi::gee_hashable_hash()
        }
    }

    //fn equal_to(object: /*Unimplemented*/Fundamental: Pointer) -> bool {
    //    unsafe { TODO: call ffi:gee_hashable_equal_to() }
    //}
}

impl fmt::Display for Hashable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Hashable")
    }
}
