// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7c3d3f5+)
// DO NOT EDIT

use crate::Collection;
use crate::Queue;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Deque(Interface<ffi::GeeDeque>) @requires Queue, Collection;

    match fn {
        get_type => || ffi::gee_deque_get_type(),
    }
}

pub const NONE_DEQUE: Option<&Deque> = None;

pub trait DequeExt: 'static {
    //#[doc(alias = "gee_deque_offer_head")]
    //fn offer_head(element: /*Unimplemented*/Fundamental: Pointer) -> bool;

    //#[doc(alias = "gee_deque_peek_head")]
    //fn peek_head() -> /*Unimplemented*/Option<Fundamental: Pointer>;

    //#[doc(alias = "gee_deque_poll_head")]
    //fn poll_head() -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "gee_deque_drain_head")]
    fn drain_head<P: IsA<Collection>>(recipient: &P, amount: i32) -> i32;

    //#[doc(alias = "gee_deque_offer_tail")]
    //fn offer_tail(element: /*Unimplemented*/Fundamental: Pointer) -> bool;

    //#[doc(alias = "gee_deque_peek_tail")]
    //fn peek_tail() -> /*Unimplemented*/Option<Fundamental: Pointer>;

    //#[doc(alias = "gee_deque_poll_tail")]
    //fn poll_tail() -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "gee_deque_drain_tail")]
    fn drain_tail<P: IsA<Collection>>(recipient: &P, amount: i32) -> i32;
}

impl<O: IsA<Deque>> DequeExt for O {
    //fn offer_head(element: /*Unimplemented*/Fundamental: Pointer) -> bool {
    //    unsafe { TODO: call ffi:gee_deque_offer_head() }
    //}

    //fn peek_head() -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gee_deque_peek_head() }
    //}

    //fn poll_head() -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gee_deque_poll_head() }
    //}

    fn drain_head<P: IsA<Collection>>(recipient: &P, amount: i32) -> i32 {
        unsafe {
            ffi::gee_deque_drain_head(recipient.as_ref().to_glib_none().0, amount)
        }
    }

    //fn offer_tail(element: /*Unimplemented*/Fundamental: Pointer) -> bool {
    //    unsafe { TODO: call ffi:gee_deque_offer_tail() }
    //}

    //fn peek_tail() -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gee_deque_peek_tail() }
    //}

    //fn poll_tail() -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gee_deque_poll_tail() }
    //}

    fn drain_tail<P: IsA<Collection>>(recipient: &P, amount: i32) -> i32 {
        unsafe {
            ffi::gee_deque_drain_tail(recipient.as_ref().to_glib_none().0, amount)
        }
    }
}

impl fmt::Display for Deque {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Deque")
    }
}
