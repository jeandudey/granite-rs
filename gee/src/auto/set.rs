// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7c3d3f5+)
// DO NOT EDIT

use crate::Collection;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Set(Interface<ffi::GeeSet>) @requires Collection;

    match fn {
        get_type => || ffi::gee_set_get_type(),
    }
}

impl Set {
    //#[doc(alias = "gee_set_empty")]
    //pub fn empty(g_type: glib::types::Type, g_dup_func: /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer) -> /*Unimplemented*/Fundamental: Pointer) -> Option<Set> {
    //    unsafe { TODO: call ffi:gee_set_empty() }
    //}
}

pub const NONE_SET: Option<&Set> = None;

pub trait SetExt: 'static {
    #[doc(alias = "gee_set_get_read_only_view")]
    fn get_read_only_view() -> Option<Set>;

    fn connect_property_read_only_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Set>> SetExt for O {
    fn get_read_only_view() -> Option<Set> {
        unsafe {
            from_glib_full(ffi::gee_set_get_read_only_view())
        }
    }

    fn connect_property_read_only_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_only_view_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GeeSet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<Set>
        {
            let f: &F = &*(f as *const F);
            f(&Set::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::read-only-view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_read_only_view_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Set")
    }
}
