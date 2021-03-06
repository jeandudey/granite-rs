// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7c3d3f5+)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Iterator(Interface<ffi::GeeIterator>);

    match fn {
        get_type => || ffi::gee_iterator_get_type(),
    }
}

impl Iterator {
    //#[doc(alias = "gee_iterator_unfold")]
    //pub fn unfold<P: IsA<Lazy>>(a_type: glib::types::Type, a_dup_func: /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer) -> /*Unimplemented*/Fundamental: Pointer, f: /*Unimplemented*/Fn(glib::types::Type, /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer) -> /*Unimplemented*/Fundamental: Pointer, &Fn() + 'static) -> Lazy, f_target: /*Unimplemented*/Fundamental: Pointer, current: &P) -> Option<Iterator> {
    //    unsafe { TODO: call ffi:gee_iterator_unfold() }
    //}

    //#[doc(alias = "gee_iterator_concat")]
    //pub fn concat<P: IsA<Iterator>>(g_type: glib::types::Type, g_dup_func: /*Unimplemented*/Fn(/*Unimplemented*/Fundamental: Pointer) -> /*Unimplemented*/Fundamental: Pointer, iters: &P) -> Option<Iterator> {
    //    unsafe { TODO: call ffi:gee_iterator_concat() }
    //}
}

pub const NONE_ITERATOR: Option<&Iterator> = None;

pub trait IteratorExt: 'static {
    #[doc(alias = "gee_iterator_next")]
    fn next() -> bool;

    #[doc(alias = "gee_iterator_has_next")]
    fn has_next() -> bool;

    //#[doc(alias = "gee_iterator_get")]
    //fn get() -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "gee_iterator_remove")]
    fn remove();

    #[doc(alias = "gee_iterator_get_valid")]
    fn get_valid() -> bool;

    #[doc(alias = "gee_iterator_get_read_only")]
    fn get_read_only() -> bool;

    fn connect_property_valid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Iterator>> IteratorExt for O {
    fn next() -> bool {
        unsafe {
            from_glib(ffi::gee_iterator_next())
        }
    }

    fn has_next() -> bool {
        unsafe {
            from_glib(ffi::gee_iterator_has_next())
        }
    }

    //fn get() -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gee_iterator_get() }
    //}

    fn remove() {
        unsafe {
            ffi::gee_iterator_remove();
        }
    }

    fn get_valid() -> bool {
        unsafe {
            from_glib(ffi::gee_iterator_get_valid())
        }
    }

    fn get_read_only() -> bool {
        unsafe {
            from_glib(ffi::gee_iterator_get_read_only())
        }
    }

    fn connect_property_valid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_valid_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GeeIterator, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<Iterator>
        {
            let f: &F = &*(f as *const F);
            f(&Iterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::valid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_valid_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_only_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GeeIterator, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<Iterator>
        {
            let f: &F = &*(f as *const F);
            f(&Iterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::read-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_read_only_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Iterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Iterator")
    }
}
