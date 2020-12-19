use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

use gtk::Widget;

glib::wrapper! {
    pub struct Welcome(Object<ffi::GraniteWidgetsWelcome, ffi::GraniteWidgetsWelcomeClass>) @extends Widget;

    match fn {
        get_type => || ffi::granite_widgets_welcome_get_type(),
    }
}

impl Welcome {
    #[doc(alias = "granite_widgets_welcome_new")]
    pub fn new(title_text: &str, subtitle_text: &str) -> Welcome {
        unsafe {
            from_glib_none(ffi::granite_widgets_welcome_new(title_text.to_glib_none().0, subtitle_text.to_glib_none().0))
        }
    }
}

pub trait WelcomeExt: 'static {
    #[doc(alias = "granite_widgets_welcome_get_title")]
    fn get_title() -> Option<glib::GString>;

    #[doc(alias = "granite_widgets_welcome_set_title")]
    fn set_title(value: &str);

    #[doc(alias = "granite_widgets_welcome_get_subtitle")]
    fn get_subtitle() -> Option<glib::GString>;

    #[doc(alias = "granite_widgets_welcome_set_subtitle")]
    fn set_subtitle(value: &str);

    #[doc(alias = "granite_widgets_welcome_set_item_visible")]
    fn set_item_visible(index: u32, val: bool);

    #[doc(alias = "granite_widgets_welcome_remove_item")]
    fn remove_item(index: u32);

    #[doc(alias = "granite_widgets_welcome_set_item_sensitivity")]
    fn set_item_sensitivity(index: u32, val: bool);

    #[doc(alias = "granite_widgets_welcome_append")]
    fn append(icon_name: Option<&str>, option_text: &str, description_text: &str) -> i32;

    //#[doc(alias = "granite_widgets_welcome_append_with_pixbuf")]
    //fn append_with_pixbuf(pixbuf: /*Ignored*/&gdk_pixbuf::Pixbuf, option_text: &str, description_text: &str) -> i32;

    //#[doc(alias = "granite_widgets_welcome_append_with_image")]
    //fn append_with_image(image: /*Ignored*/&gtk::Image, option_text: &str, description_text: &str) -> i32;

    //#[doc(alias = "granite_widgets_welcome_get_button_from_index")]
    //fn get_button_from_index(index: i32) -> /*Ignored*/Option<WidgetsWelcomeButton>;

    fn connect_activated<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Welcome>> WelcomeExt for O {
    fn get_title() -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::granite_widgets_welcome_get_title())
        }
    }

    fn set_title(value: &str) {
        unsafe {
            ffi::granite_widgets_welcome_set_title(value.to_glib_none().0);
        }
    }

    fn get_subtitle() -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::granite_widgets_welcome_get_subtitle())
        }
    }

    fn set_subtitle(value: &str) {
        unsafe {
            ffi::granite_widgets_welcome_set_subtitle(value.to_glib_none().0);
        }
    }

    fn set_item_visible(index: u32, val: bool) {
        unsafe {
            ffi::granite_widgets_welcome_set_item_visible(index, val.to_glib());
        }
    }

    fn remove_item(index: u32) {
        unsafe {
            ffi::granite_widgets_welcome_remove_item(index);
        }
    }

    fn set_item_sensitivity(index: u32, val: bool) {
        unsafe {
            ffi::granite_widgets_welcome_set_item_sensitivity(index, val.to_glib());
        }
    }

    fn append(icon_name: Option<&str>, option_text: &str, description_text: &str) -> i32 {
        unsafe {
            ffi::granite_widgets_welcome_append(icon_name.to_glib_none().0, option_text.to_glib_none().0, description_text.to_glib_none().0)
        }
    }

    //fn append_with_pixbuf(pixbuf: /*Ignored*/&gdk_pixbuf::Pixbuf, option_text: &str, description_text: &str) -> i32 {
    //    unsafe { TODO: call ffi:granite_widgets_welcome_append_with_pixbuf() }
    //}

    //fn append_with_image(image: /*Ignored*/&gtk::Image, option_text: &str, description_text: &str) -> i32 {
    //    unsafe { TODO: call ffi:granite_widgets_welcome_append_with_image() }
    //}

    //fn get_button_from_index(index: i32) -> /*Ignored*/Option<WidgetsWelcomeButton> {
    //    unsafe { TODO: call ffi:granite_widgets_welcome_get_button_from_index() }
    //}

    fn connect_activated<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activated_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut ffi::GraniteWidgetsWelcome, index: libc::c_int, f: glib::ffi::gpointer)
            where P: IsA<Welcome>
        {
            let f: &F = &*(f as *const F);
            f(&Welcome::from_glib_borrow(this).unsafe_cast_ref(), index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(activated_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GraniteWidgetsWelcome, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<Welcome>
        {
            let f: &F = &*(f as *const F);
            f(&Welcome::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_title_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GraniteWidgetsWelcome, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<Welcome>
        {
            let f: &F = &*(f as *const F);
            f(&Welcome::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_subtitle_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Welcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Welcome")
    }
}
